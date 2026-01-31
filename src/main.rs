use axum::{
    Router,
    routing::{get, post},
};

use tower_http::services::ServeDir;

use tera::Tera;

use lazy_static::lazy_static;

use std::sync::Arc;

use serenity::all::Http;
use serenity::prelude::*;

use clap::Parser;

mod cli;
mod config;
mod database;
mod events;
mod handlers;
mod recaptcha_verify;

use crate::{cli::CliSubcommands, config::CONFIG};
use cli::Cli;
use events::GuildEventHandler;
use recaptcha_verify::*;

lazy_static! {
    /// Initialize the templating engine
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template("event_grid", include_str!("templates/event_grid.html")).unwrap();
        tera
    };

    pub static ref CLI_ARGS: Cli = {
        Cli::parse()
    };
}

/// Things that route handlers need access to.
#[derive(Clone)]
struct AppState {
    /// Serenity http struct for interacting with Discord API
    http: Arc<Http>,
}

#[tokio::main]
async fn main() {
    match &CLI_ARGS.subcommand {
        Some(CliSubcommands::InitDatabase { dir: _ }) => {
            // TODO: Use the dir field. Currently, the program uses CONFIG.database_path instead

            println!("Initializing database at {:?}", CONFIG.database_path);

            database::create_database().unwrap();

            println!("Success");
        }
        None => start_server().await,
    };
}

async fn start_server() {
    let discord_intents = GatewayIntents::GUILD_SCHEDULED_EVENTS;
    let mut discord_client = Client::builder(&CONFIG.discord.bot_token, discord_intents)
        .event_handler(GuildEventHandler)
        .await
        .unwrap();

    let state = AppState {
        http: Arc::clone(&discord_client.http),
    };

    // Start the client in a new tokio worker so we don't block the main thread.
    tokio::spawn(async move {
        discord_client.start().await.unwrap();
    });

    let router = Router::new()
        .route("/api/get_events", get(get(handlers::get_events)))
        .route("/api/generate_invite", post(handlers::generate_invite))
        .fallback_service(ServeDir::new(&CONFIG.static_site_path))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, router).await.unwrap(); // axum::serve blocks the main thread
}
