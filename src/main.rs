use axum::{
    Router,
    routing::{get, post},
};

use tower_http::services::ServeDir;

use tera::Tera;

use lazy_static::lazy_static;

use std::{env, path::PathBuf, sync::Arc};

use serenity::all::Http;
use serenity::prelude::*;

use clap::Parser;

mod cli;
mod config;
mod events;
mod handlers;
mod recaptcha_verify;

use events::GuildEventHandler;
use crate::{config::CONFIG};
use recaptcha_verify::*;
use cli::Cli;

lazy_static! {
    /// Initialize the templating engine
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template("event_grid", include_str!("templates/event_grid.html")).unwrap();
        tera
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
    let args = Cli::parse();

    let static_site_path: PathBuf = match args.static_site_path {
        Some(s) => s,
        None => "/var/sdnc/www".into(),
    };


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

    // discord_client.start().await.unwrap();
    // discord_client.start_autosharded().await.unwrap();

    let router = Router::new()
        .route("/api/get_events", get(get(handlers::get_events)))
        .route("/api/generate_invite", post(handlers::generate_invite))
        .fallback_service(ServeDir::new(&static_site_path))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, router).await.unwrap(); // axum::serve blocks the main thread
}


