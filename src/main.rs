use axum::{
    extract::State, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router
};

use tower_http::services::ServeDir;

use serde::{Deserialize, Serialize};

use std::sync::Arc;

use serenity::all::{ChannelId, CreateInvite, GuildId};
use serenity::prelude::*;

mod recaptcha_verify;
use recaptcha_verify::*;

use crate::config::CONFIG;

mod config;

/// Things that route handlers need access to.
#[derive(Clone)]
struct AppState {
    discord_client: Arc<Client>,
}

#[tokio::main]
async fn main() {
    let discord_intents = GatewayIntents::GUILD_SCHEDULED_EVENTS;
    let discord_client = Client::builder(&CONFIG.discord.bot_token, discord_intents)
        .await
        .unwrap();

    let state = AppState {
        discord_client: Arc::new(discord_client),
    };

    let router = Router::new()
        .route("/api/get_events", get(get(get_events)))
        .route("/api/generate_invite", post(generate_invite))
        .fallback_service(ServeDir::new("/home/markg/Documents/Code/sdnc/www/public"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize)]
struct InviteForm {
    #[serde(rename = "g-recaptcha-response")]
    g_recaptcha_response: String,
}

async fn generate_invite(
    State(state): State<AppState>,
    Form(invite_form): Form<InviteForm>,
) -> impl IntoResponse {
    println!(
        "Recaptcha response token: {}",
        invite_form.g_recaptcha_response
    );

    match recaptcha_verify(&invite_form.g_recaptcha_response).await {
        Err(e) => return format!("Error validating captcha: {e}").into_response(),
        Ok(false) => return "Invalid captcha".into_response(),
        Ok(true) => { /* Continue */ }
    }

    println!("Captcha passed. Generating invite...");

    // Generate a single use discord invite
    let channel = ChannelId::new(CONFIG.discord.channel_id);

    let invite = channel
        .create_invite(
            &state.discord_client.http,
            CreateInvite::new().max_age(60).max_uses(1),
        )
        .await
        .unwrap();

    // Redirect directly to the new invite link
    Redirect::to(&invite.url()).into_response()
}

async fn get_events(State(state): State<AppState>) -> Json<Vec<EventDetails>> {
    let guild = GuildId::new(CONFIG.discord.guild_id);
    let events = guild
        .scheduled_events(&state.discord_client.http, true)
        .await
        .unwrap();

    let events: Vec<EventDetails> = events
        .into_iter()
        .map(|e| EventDetails { 
            name: e.name,
            start_time: (),
            end_time: (),
            description: e.description,
            location: e.metadata.unwrap().location,
            rsvps: e.user_count.unwrap_or(0),
            discord_link: format!("https://discord.com/events/{}/{}", e.guild_id, e.id)
        })
        .collect();

    axum::Json(events)
}

/// API schema for `get_events` endpoint
#[derive(Serialize)]
struct EventDetails {
    name: String,
    start_time: (),
    end_time: (),
    description: Option<String>,
    location: Option<String>,
    rsvps: u64,

    /// The discord URL for the event. Should look like "https://discord.com/events/1224949123141210173/1423060568952017120".
    discord_link: String,
}
