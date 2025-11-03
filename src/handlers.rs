//! This module contains the handler functions for the axum routes

use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use serenity::all::{ChannelId, CreateInvite, GuildId};

use crate::{
    AppState, TERA, config::CONFIG, events::EventDetails, recaptcha_verify::recaptcha_verify,
};

/// Handler for /api/get_events route. Fetches list of events from Discord guild
/// and fills them into an html template.
pub async fn get_events(State(state): State<AppState>) -> impl IntoResponse {
    let guild = GuildId::new(CONFIG.discord.guild_id);
    let events = guild.scheduled_events(&state.http, true).await.unwrap(); // TODO: Handle this unwrap gracefully

    let events: Vec<EventDetails> = events.into_iter().map(|e| e.into()).collect();

    // Uncomment this to return Json instead
    // axum::Json(events)

    let mut context = tera::Context::new();
    context.insert("events", &events);

    // This unwrap should not panic if there are no bugs in the template.
    let body = TERA.render("event_grid", &context).unwrap();

    // Return the output of the template as HTML content type
    Html(body)
}

/// Form data for /api/generate_invite endpoint
#[derive(Deserialize)]
pub struct InviteForm {
    #[serde(rename = "g-recaptcha-response")]
    g_recaptcha_response: String,
}

/// Handler for /api/generate_invite endpoint. Verifies reCAPTCHA token and redirects
/// to a new discord invite link if the token is valid.
pub async fn generate_invite(
    State(state): State<AppState>,
    Form(invite_form): Form<InviteForm>,
) -> impl IntoResponse {
    match recaptcha_verify(&invite_form.g_recaptcha_response).await {
        Err(e) => return format!("Error validating captcha: {e}").into_response(),
        Ok(false) => return "Invalid captcha".into_response(),
        Ok(true) => { /* Continue */ }
    }

    println!("Captcha passed. Generating invite.");

    // Generate a single use discord invite
    let channel = ChannelId::new(CONFIG.discord.channel_id);

    let invite = channel
        .create_invite(&state.http, CreateInvite::new().max_age(60).max_uses(1))
        .await
        .unwrap(); // TODO: Handle this unwrap gracefully

    // Redirect directly to the new invite link
    Redirect::to(&invite.url()).into_response()
}
