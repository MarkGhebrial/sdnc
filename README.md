# sandiegonerfclub.org

This is the source code for [sandiegonerfclub.org](https://sandiegonerfclub.org).
It's a mostly static zola-generated site with a simple axum-based backend. It's an
extremely cursed way to make a website, please don't ever copy this architecture.

# Deploying

When deploying for the first time, run `bash install.sh`

When updating the deployment, run `bash update.sh`

# Developing

To run the server without installing it:
```bash
cd www
zola build # must be version 0.22 or earlier
cd ..
cp example_config.toml config.toml

# In config.toml, set the following:
#   static_site_path = "www/public"
#   database_path = "database.sqlite"

diesel migration run --database-url database.sqlite

cargo r -- -c ./config.toml
```
