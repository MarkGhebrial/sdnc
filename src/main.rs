use axum::{
    Form, Router,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};

use tower_http::services::ServeDir;

use serde::Deserialize;

use std::env;

use serenity::all::GuildId;
use serenity::prelude::*;

mod recaptcha_verify;
use recaptcha_verify::*;

use crate::config::CONFIG;

mod config;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let discord_intents = GatewayIntents::GUILD_SCHEDULED_EVENTS | GatewayIntents::GUILD_INVITES;
    let discord_client = Client::builder(&token, discord_intents).await.unwrap();

    let router = Router::new()
        .route("/api/get_events", get(|| async { "Hello, World!" }))
        .route("/api/generate_invite", post(generate_invite))
        .fallback_service(ServeDir::new("/home/markg/Documents/Code/sdnc/www/public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize)]
struct InviteForm {
    #[serde(rename = "g-recaptcha-response")]
    g_recaptcha_response: String,
}

async fn generate_invite(Form(invite_form): Form<InviteForm>) -> Response {
    println!(
        "Recaptcha response token: {}",
        invite_form.g_recaptcha_response
    );

    match recaptcha_verify(&invite_form.g_recaptcha_response).await {
        Err(_) | Ok(false) => "Invalid captcha".into_response(),
        Ok(true) => {
            println!("Captcha passed");

            Redirect::to("https://google.com").into_response()
        }
    }

    // recaptcha_verify(&invite_form.g_recaptcha_response).await.unwrap_or_else(|_| {
    //     return "Invalid captcha".into_response();
    // });

    // Redirect::to("https://google.com").into_response()
}

async fn get_events(client: &Client) -> String {
    let guild = GuildId::new(CONFIG.discord.guild_id);
    let events = guild
        .scheduled_events(client.http.clone(), true)
        .await
        .unwrap();

    for event in events {
        println!("Event name: {}", event.name);
        println!("Event time: {} - {:?}", event.start_time, event.end_time);
        println!("Event location: {:?}", event.metadata.unwrap().location);
        println!("Number of RSVPs: {:?}", event.user_count);

        println!("Description: {:?}", event.description)
        // println!("{event:?}");
    }

    "<p>this is a test</p>".to_string()
}

struct EventDetails {
    name: Option<String>,
    start_time: (),
    end_time: (),
    location: Option<String>,
    rsvps: usize,
}
