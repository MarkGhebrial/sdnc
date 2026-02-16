#! /bin/bash

git pull

cargo build --release

# Apply database migrations
cargo r -- init-database -c "/var/sdnc/config.toml"

echo "Building static site"
cd www
zola build
sudo rm -r /var/sdnc/www/*
sudo cp -a public/. /var/sdnc/www/
cd ..

# Install systemd service file
echo "Installing systemd service file."
sudo systemctl stop sdnc
# Copy over the new systemd file
sudo cp sdnc.service /etc/systemd/system/sdnc.service
sudo systemctl daemon-reload
# Copy the new executable over
sudo cp target/release/sdnc /var/sdnc/sdnc
sudo systemctl stop sdnc
sudo systemctl enable sdnc
sudo systemctl start sdnc