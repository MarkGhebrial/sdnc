use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use chrono::Utc;
use chrono_tz::America;
use diesel::prelude::*;
use serde::Serialize;

use crate::{
    AppState, TERA, config::CONFIG, database::{self, models, schema}
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
    ///
    /// This entry is None for events that are no longer on the discord server (i.e
    /// events that are in the past.)
    discord_link: Option<String>,
}

// TODO: This function needs some cleanup
async fn event_helper(get_past_events: bool) -> impl IntoResponse {
    let mut conn = database::connect_to_database();

    let query_result = {
        use schema::events::dsl::*;

        let current_time = Utc::now();

        // Fetch the events from the database
        let query_result = if get_past_events {
            events
                .select(models::Event::as_select())
                .filter(end_time.le(current_time))
                .filter(guild_id.eq(CONFIG.discord.guild_id as f64))
                .order(start_time)
                .load(&mut conn)
        } else {
            events
                .select(models::Event::as_select())
                .filter(end_time.gt(current_time))
                .filter(guild_id.eq(CONFIG.discord.guild_id as f64))
                .order(start_time)
                .load(&mut conn)
        };

        match query_result {
            Ok(r) => r,
            Err(_) => return Html("<p>Error getting events.</p>".to_owned()),
        }
    };

    let events: Vec<EventDetails> = query_result
        .into_iter()
        .map(|e| EventDetails {
            name: e.event_name,
            start_time: e.start_time
                .with_timezone(&America::Los_Angeles)
                .format("%m/%d/%Y %l:%M%P")
                .to_string(),
            end_time: e.end_time.map(|s| {
                s.with_timezone(&America::Los_Angeles)
                    .format("%m/%d/%Y %l:%M%P")
                    .to_string()
            }),
            description: e.event_description,
            location: e.event_location,
            rsvps: e.rsvps,
            discord_link: if !get_past_events {
                Some(format!(
                    "https://discord.com/events/{}/{}",
                    e.guild_id, e.event_id
                ))
            } else {
                None
            },
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

/// Handler for /api/get_events route. Fetches list of events from Discord guild
/// and fills them into an html template.
pub async fn get_events(State(_state): State<AppState>) -> impl IntoResponse {
    event_helper(false).await
}

pub async fn get_previous_events(State(_state): State<AppState>) -> impl IntoResponse {
    event_helper(true).await
}
