#! /bin/bash

git pull

sudo mkdir /var/sdnc
sudo mkdir /var/sdnc/www

cargo build --release
sudo cp target/release/sdnc /var/sdnc/sdnc

echo "Building static site"
cd www
zola build
sudo cp -r public /var/sdnc/www
cd ..

echo "Creating config file."
sudo cp example_config.toml /var/sdnc/config.toml

# Install systemd service file
echo "Installing systemd service file."
sudo cp sdnc.service /etc/systemd/system/sdnc.service
sudo systemctl daemon-reload

echo "Done. Edit /var/sdnc/config.toml, then start and enable the sdnc service."
# sudo systemctl enable sdnc
# sudo systemctl start sdnc