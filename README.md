# pdock

A simple dock application for both X11 and Wayland written in Rust using GTK4. 

![Preview of pdock](https://github.com/TadaTeruki/pdock/assets/69315285/fd5f6eae-fa64-4528-b0f6-f10008684413)

## Notice

For now, this application is just a toy project for myself. But if you like this project and want to contribute, please feel free to send issues and pull requests.

## Installation guide

At first, this requires a development environment for Rust to build. Please refer to the [official document](https://www.rust-lang.org/tools/install) for installation.

### 1.run `install.sh`

```
$ sh install.sh
```

Be cafeful that this script will install the application to `/usr/local/bin`. If you want to install to another directory, please edit the script.

### 2.configure your desktop environment to execute pdock

This application doesn't set its position and size by itself because of using GTK4. So you need to configure your desktop environment to set the initial position and size of the dock and execute the dock.

This is an example for [sway](https://github.com/swaywm/sway):

```
set $pdock pdock

exec_always killall -q pdock
exec_always while pgrep -x pdock >/dev/null; do sleep 1; done

for_window [app_id="dev.peruki.pdock"] {
  floating enable
  sticky enable
  move to output $screen1
  floating_minimum_size 1 x 1 
  border none
  window_type _NET_WM_WINDOW_TYPE_DOCK
}

exec_always $pdock
```

### 3.edit config file

After running `install.sh` for the first time, the config file will be created at `~/.config/pdock/config`. You can edit this file to configure the dock.

Here is properties of the config file:
 - `apps`: the list of applications to be displayed on the dock. If you want to add more, see `/usr/share/applications/` and add the name of the desktop file to the array.
 - `button_height`: the size of the button.

This is an example of the config file:
```
{
  "apps": [
    "firefox",
    "org.gnome.gedit",
    "kitty",
    "org.gnome.Nautilus",
    "code"
  ],
  "button_height": 65
}
```

Also, if you want to change the style, you can edit `~/.config/pdock/style.css`.

## License

MIT

