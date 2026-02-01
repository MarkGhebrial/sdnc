use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
};

use clap::Parser;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::cli::Cli;

lazy_static! {
     pub static ref CONFIG: Config = {
        let args = Cli::parse();
        // If a path is not specified in the command line arguments, use the current working directory
        let config_file_path = args.config_path.unwrap_or(env::current_dir().unwrap());

        // Read the file
        let path = Path::new(&config_file_path);
        let toml = fs::read_to_string(path).unwrap_or_else(|_e| {
            println!("Could not read config file at {}.", path.to_string_lossy());
            exit(-1);
        });

        toml::from_str(&toml).expect("Couldn't parse config file. Is the syntax correct?")
    };
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_static_site_path")]
    pub static_site_path: PathBuf,
    #[serde(default = "default_database_url")]
    pub database_url: String,

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

    pub channel_id: u64,
}

fn default_static_site_path() -> PathBuf {
    "/var/sdnc/www".into()
}
fn default_database_url() -> String {
    "sdnc.sqlite".into()
}
