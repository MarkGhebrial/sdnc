#! /bin/bash

git pull

cargo build --release
sudo cp target/release/sdnc /var/sdnc/sdnc

echo "Building static site"
cd www
zola build
sudo cp -r public /var/sdnc/www
cd ..

# Install systemd service file
echo "Installing systemd service file."
sudo cp sdnc.service /etc/systemd/system/sdnc.service
sudo systemctl daemon-reload
sudo systemctl enable sdnc
sudo systemctl start sdnc