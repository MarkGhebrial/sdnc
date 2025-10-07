# sandiegonerfclub.org

This is the source code for [sandiegonerfclub.org](https://sandiegonerfclub.org).
It's a mostly static zola-generated site with a simple axum-based backend.

# Deploying

When deploying for the first time:
1. `cp example_config.toml config.toml`, then change *all* the default options.
   Every item in the example config is mandatory.
2. `sudo cp sdnc.service /etc/systemd/system/sdnc.service`
3. `sudo systemctl enable sndc`
4. Set up Caddy proxy.
4. Set up update.sh cron job to run every 5 minutes.

When deploying for the first time and when updating the deployment:
1. `cargo build --release`
2. `cd www` then `zola build` then `cd ..`
3. `cp target/release/sdnc /usr/local/bin/sdnc`
4. 
