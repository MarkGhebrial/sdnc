use std::{fs, path::Path};

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = {
        // Read the file
        let path = Path::new("/home/markg/Documents/Code/sdnc/config.toml");
        let toml = fs::read_to_string(path).expect("Couldn't read config file. Is the path correct?");

        toml::from_str(&toml).expect("Couldn't parse config file. Is the syntax correct?")
    };
}

#[derive(Deserialize)]
pub struct Config {
    pub google: GoogleApiConfig,
    pub discord: DiscordConfig,
}

#[derive(Deserialize)]
pub struct GoogleApiConfig {
    pub recaptcha_site_key: String,
    pub google_api_key: String,
    pub google_cloud_project_name: String,
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub bot_token: String,

    pub guild_id: u64,
}
