use axum::Form;
use axum::response::Redirect;
use axum::{extract::State, response::IntoResponse};
use serde::Deserialize;
use serenity::all::{ChannelId, CreateInvite};

use crate::AppState;
use crate::config::CONFIG;
use crate::recaptcha_verify;

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
