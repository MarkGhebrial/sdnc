use std::path::PathBuf;

use clap::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name="FILE")]
    pub config_path: Option<PathBuf>,

    #[arg(short, long, value_name="DIR")]
    pub static_site_path: Option<PathBuf>,
}