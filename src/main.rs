//! pdock is a simple dock application written in Rust using GTK4.

use gtk::gdk::Display;
use gtk::glib;
use gtk::prelude::*;
use gtk::CssProvider;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use xdg::BaseDirectories;

// application ID
static APP_ID: &str = "dev.peruki.pdock";

/// Application struct holding the file path to icon and command of an application.
#[derive(Debug, Clone)]
struct App {
    icon: Option<String>,
    command: Option<String>,
}

/// main function
fn main() -> glib::ExitCode {
    gtk::init().expect("Failed to initialize GTK.");
    let application = gtk::Application::new(Some(APP_ID), Default::default());

    // load css file
    let provider = CssProvider::new();
    let base_dirs = BaseDirectories::with_prefix("pdock").unwrap();
    let style_path = base_dirs.find_config_file("style.css").unwrap();
    provider.load_from_path(style_path);
    let display = Display::default().expect("Could not connect to a display.");

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // set up application
    application.connect_activate(build_ui);

    // run application
    application.run()
}

/// Build the UI of the application.
fn build_ui(application: &gtk::Application) {
    // read json file form config directory
    let base_dirs = BaseDirectories::with_prefix("pdock").unwrap();
    let config_path = base_dirs.find_config_file("config").unwrap();
    let config_content = fs::read_to_string(&config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    let app_names = config["apps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect::<Vec<&str>>();
    let button_height = config["button_height"].as_i64().unwrap() as i32;

    let base_dirs = BaseDirectories::with_prefix("applications").unwrap();
    let apps = {
        let mut apps = Vec::new();
        for app_name in &app_names {
            let mut app = App {
                icon: None,
                command: None,
            };

            let entry_path = base_dirs
                .find_data_file(format!("{}.desktop", app_name))
                .unwrap();
            let content = fs::read_to_string(&entry_path).unwrap();
            let lines: Vec<&str> = content.lines().collect();

            for line in lines {
                if line.starts_with("Icon=") {
                    app.icon = find_icon(line[5..].into());
                }
                if line.starts_with("Exec=") {
                    app.command = Some(line[5..].split(' ').next().unwrap().into());
                }
            }
            apps.push(app);
        }
        apps
    };

    // create dock window that contains all the items
    let dock_window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("pdock")
        .decorated(false)
        .resizable(false)
        .css_classes(["pdock"])
        .build();

    // create box that contains buttons
    let button_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    button_box.set_css_classes(&["button"]);

    // create buttons
    let mut first = true;
    for app in apps {
        if first {
            first = false;
        } else {
            // add separator
            let separator = gtk::Separator::new(gtk::Orientation::Vertical);
            button_box.append(&separator);
        }

        let button = gtk::Button::new();
        button.set_css_classes(&["app"]);
        if let Some(icon) = app.icon {
            let image = gtk::Image::from_file(&icon);
            button.set_child(Some(&image));
        }
        if let Some(command) = app.command {
            button.connect_clicked(move |_| {
                std::process::Command::new(&command)
                    .spawn()
                    .expect("failed to execute process");
            });
        }
        button.set_size_request(button_height, button_height);
        button_box.append(&button);
    }

    // create box that contains button box and expand button
    let expand_box = gtk::Box::new(gtk::Orientation::Horizontal, 3);
    expand_box.append(&button_box);

    // create expand button
    // expand button is used to hide and show the button box
    let button_exp = gtk::Button::new();
    button_exp.set_css_classes(&["expand"]);
    button_exp.set_height_request(button_height * app_names.len() as i32);

    // hide button box
    let hide_box = |bbox: &gtk::Box, bexp: &gtk::Button| {
        bbox.hide();
        bexp.add_css_class("collapsed");
        bexp.set_label("");
    };

    // show button box
    let show_box = |bbox: &gtk::Box, bexp: &gtk::Button| {
        bbox.show();
        bexp.remove_css_class("collapsed");
        bexp.set_label("<");
    };

    // set up click event for expand button
    {
        let button_box = button_box.clone();
        button_exp.connect_clicked(move |b| {
            if button_box.is_visible() {
                hide_box(&button_box, b);
            } else {
                show_box(&button_box, b);
            }
        });
    }

    // set up mouse motion event controller
    let dock_mouse = gtk::EventControllerMotion::new();
    {
        let button_box = button_box.clone();
        let button_exp = button_exp.clone();
        dock_mouse.connect_motion(move |_, _, _| {
            if !button_box.is_visible() {
                show_box(&button_box, &button_exp);
            }
        });
    }
    dock_window.add_controller(dock_mouse);

    // at first, the button box is hidden
    hide_box(&button_box, &button_exp);

    // set up dock window
    expand_box.append(&button_exp);
    dock_window.set_child(Some(&expand_box));
    dock_window.present();
}

/// Find the icon file of the given icon name.
fn find_icon(icon_name: &str) -> Option<String> {
    let icon_dirs = [
        Path::new("/usr/share/icons"),
        &Path::new(&dirs::home_dir().unwrap()).join(".icons"),
    ];
    let icon_exts = ["png", "svg", "xpm"];

    for dir in &icon_dirs {
        for ext in &icon_exts {
            let icon_file_name = format!("{}.{}", icon_name, ext);
            for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                if entry.file_name().to_string_lossy() == icon_file_name {
                    return Some(entry.path().to_path_buf().to_string_lossy().to_string());
                }
            }
        }
    }
    None
}
