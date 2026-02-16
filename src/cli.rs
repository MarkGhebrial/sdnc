use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long, value_name = "FILE")]
    pub config_path: PathBuf,

    // #[arg(short = 's', long, value_name = "DIR")]
    // pub static_site_path: Option<PathBuf>,

    // #[arg(short = 'd', long, value_name = "DIR")]
    // pub sqlite_database_path: Option<PathBuf>,
    #[command(subcommand)]
    pub subcommand: Option<CliSubcommands>,
}

#[derive(Subcommand)]
pub enum CliSubcommands {
    InitDatabase,
}
