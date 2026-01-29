use axum::{extract::State, response::{Html, IntoResponse}};
use serde::Serialize;
use serenity::all::GuildId;

use crate::{AppState, TERA, config::CONFIG};


/// Struct for "event_grid" template data
#[derive(Serialize)]
pub struct EventDetails {
    name: String,
    start_time: String,
    end_time: Option<String>,
    description: Option<String>,
    location: Option<String>,
    rsvps: u64,

    /// The discord URL for the event. Should look like "https://discord.com/events/1224949123141210173/1423060568952017120".
    discord_link: String,
}

/// Handler for /api/get_events route. Fetches list of events from Discord guild
/// and fills them into an html template.
pub async fn get_events(State(state): State<AppState>) -> impl IntoResponse {
    let guild = GuildId::new(CONFIG.discord.guild_id);
    let events = guild.scheduled_events(&state.http, true).await.unwrap(); // TODO: Handle this unwrap gracefully

    let events: Vec<EventDetails> = events
        .into_iter()
        .map(|e| EventDetails {
            name: e.name,
            start_time: format!(
                "{}",
                e.start_time
                    .with_timezone(&chrono_tz::America::Los_Angeles)
                    .format("%m/%d/%Y %l:%M%P")
            ),
            end_time: match e.end_time {
                Some(t) => Some(format!(
                    "{}",
                    t.with_timezone(&chrono_tz::America::Los_Angeles).format("%m/%d/%Y %l:%M%P")
                )),
                None => None,
            },
            description: e.description,
            location: e.metadata.unwrap().location,
            rsvps: e.user_count.unwrap_or(0),
            discord_link: format!("https://discord.com/events/{}/{}", e.guild_id, e.id),
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
