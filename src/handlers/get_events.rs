use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use chrono::DateTime;
use diesel::{associations::HasTable, prelude::*};
use serde::Serialize;

use crate::{
    AppState, TERA,
    database::{self, models, schema},
};

/// Struct for "event_grid" template data
#[derive(Serialize)]
pub struct EventDetails {
    name: String,
    start_time: String,
    end_time: Option<String>,
    description: Option<String>,
    location: Option<String>,
    rsvps: i32,

    /// The discord URL for the event. Should look like "https://discord.com/events/1224949123141210173/1423060568952017120".
    discord_link: String,
}

/// Handler for /api/get_events route. Fetches list of events from Discord guild
/// and fills them into an html template.
pub async fn get_events(State(_state): State<AppState>) -> impl IntoResponse {
    let mut conn = database::connect_to_database();

    // Fetch the events from the database
    let Ok(events) = schema::events::table::table()
        .select(models::Event::as_select())
        .load(&mut conn)
    else {
        println!("Database error");
        return Html("<p>Error getting events.</>".to_owned());
    };

    // Get the events from the discord server
    // let guild = GuildId::new(CONFIG.discord.guild_id);
    // let events = guild.scheduled_events(&state.http, true).await.unwrap(); // TODO: Handle this unwrap gracefully

    let events: Vec<EventDetails> = events
        .into_iter()
        .map(|e| EventDetails {
            name: e.event_name,
            start_time: DateTime::parse_from_rfc3339(&e.start_time)
                .unwrap()
                .format("%m/%d/%Y %l:%M%P")
                .to_string(),
            end_time: e.end_time.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .format("%m/%d/%Y %l:%M%P")
                    .to_string()
            }),
            description: e.event_description,
            location: e.event_location,
            rsvps: e.rsvps,
            discord_link: format!("https://discord.com/events/{}/{}", e.guild_id, e.event_id),
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
