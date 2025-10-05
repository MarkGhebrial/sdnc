use axum::{
    extract::State, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Router
};

use tower_http::services::ServeDir;

use tera::Tera;

use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

use std::sync::Arc;

use serenity::all::{ChannelId, CreateInvite, GuildId};
use serenity::prelude::*;

mod recaptcha_verify;
use recaptcha_verify::*;

use crate::config::CONFIG;

mod config;

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

/// Form data for /api/generate_invite endpoint
#[derive(Deserialize)]
struct InviteForm {
    #[serde(rename = "g-recaptcha-response")]
    g_recaptcha_response: String,
}

/// Handler for /api/generate_invite endpoint. Verifies reCAPTCHA token and redirects
/// to a new discord invite link if the token is valid.
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
        .unwrap(); // TODO: Handle this unwrap gracefully

    // Redirect directly to the new invite link
    Redirect::to(&invite.url()).into_response()
}

/// Struct for "event_grid" template data
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

/// Handler for /api/get_events route. Fetches list of events from Discord guild
/// and fills them into an html template.
async fn get_events(State(state): State<AppState>) -> impl IntoResponse {
    let guild = GuildId::new(CONFIG.discord.guild_id);
    let events = guild
        .scheduled_events(&state.discord_client.http, true)
        .await
        .unwrap(); // TODO: Handle this unwrap gracefully

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

    // Uncomment this to return Json instead
    // axum::Json(events)

    let mut context = tera::Context::new();
    context.insert("events", &events);

    // This unwrap should not panic if there are no bugs in the template.
    let body = TERA.render("event_grid", &context).unwrap();

    // Return the output of the template as HTML content type
    Html(body)
}

