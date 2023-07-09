#/bin/sh

# build the project
cargo build --release

# copy the binary to the directory
echo "Copying binary to /usr/local/bin ..."
sudo cp ./target/release/pdock /usr/local/bin

# create the config directory
sudo mkdir -p ~/.config/pdock

# copy style.css to the config directory
sudo cp ./resources/style.css ~/.config/pdock

# copy the config file to the config directory
sudo cp ./resources/config ~/.config/pdock
