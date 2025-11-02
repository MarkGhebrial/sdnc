# sandiegonerfclub.org

This is the source code for [sandiegonerfclub.org](https://sandiegonerfclub.org).
It's a mostly static zola-generated site with a simple axum-based backend.

# Deploying

When deploying for the first time, run `bash install.sh`

When updating the deployment, run `bash update.sh`

# Developing

The backend takes either two or zero command line arguments. Providing zero arguments
causes it to look for the config file and static site in the default locations
(/var/sdnc/config.toml and /var/sdnc/www). Providing two arguments overrides those
defaults.

To run the server without installing it:
```bash
cd www
zola build
cd ..
cargo r -- /config.toml /www/public
```
