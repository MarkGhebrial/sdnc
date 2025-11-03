use axum::{
    Router,
    routing::{get, post},
};

use tower_http::services::ServeDir;

use tera::Tera;

use lazy_static::lazy_static;

use std::{env, sync::Arc};

use serenity::all::Http;
use serenity::prelude::*;

mod config;
mod database;
mod events;
mod handlers;
mod recaptcha_verify;

use events::GuildEventHandler;

use handlers::*;
use database::*;

use crate::config::CONFIG;

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
pub struct AppState {
    /// Serenity http struct for interacting with Discord API
    http: Arc<Http>,
}

#[tokio::main]
async fn main() {
    // let mut config_file_path: String = "/var/sdnc/config.toml".to_string();
    let mut static_site_path: String = "/var/sdnc/www".to_string();

    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        // config_file_path = args[1].clone();
        static_site_path = args[2].clone();
    }

    let pool = initialize_database().await;

    let discord_intents = GatewayIntents::GUILD_SCHEDULED_EVENTS;
    let mut discord_client = Client::builder(&CONFIG.discord.bot_token, discord_intents)
        .event_handler(GuildEventHandler { db_pool: pool.clone() })
        .await
        .unwrap();

    let state = AppState {
        http: Arc::clone(&discord_client.http),
    };

    // Start the client in a new tokio worker so we don't block the main thread.
    tokio::spawn(async move {
        discord_client.start().await.unwrap();
    });

    // Initialize the axum router
    let router = Router::new()
        .route("/api/get_events", get(get(get_events)))
        .route("/api/generate_invite", post(generate_invite))
        .fallback_service(ServeDir::new(&static_site_path))
        .with_state(state);

    // Start the axum listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, router).await.unwrap(); // axum::serve blocks the main thread
}
