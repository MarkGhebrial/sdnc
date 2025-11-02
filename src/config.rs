use std::{env, fs, path::Path, process::exit};

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
     pub static ref CONFIG: Config = {
        let mut config_file_path: String = "/var/sdnc/config.toml".to_string();

        let args: Vec<String> = env::args().collect();
        if args.len() == 3 {
            config_file_path = args[1].clone();
        } else if args.len() != 1 {
            println!("WARNING: Expected 0 or 2 args");
        }

        println!("Current working directory: {}", env::current_dir().unwrap().to_string_lossy());

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
