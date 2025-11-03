use chrono::DateTime;
use chrono_tz::{America::Los_Angeles, Tz};
use serde::Serialize;
use serenity::all::{
    Context, EventHandler, GuildScheduledEventUserAddEvent, GuildScheduledEventUserRemoveEvent,
    ScheduledEvent,
};
use sqlx::{Pool, Sqlite};

/// Struct for "event_grid" template data.
#[derive(Serialize)]
pub struct EventDetails {
    pub name: String,
    pub start_time: DateTime<Tz>,
    pub end_time: Option<DateTime<Tz>>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub rsvps: u64,

    /// The discord URL for the event. Should look like "https://discord.com/events/1224949123141210173/1423060568952017120".
    pub discord_link: String,
}

/// Convert a serenity ScheduledEvent into an EventDetails for the database and API.
impl From<ScheduledEvent> for EventDetails {
    fn from(e: ScheduledEvent) -> Self {
        EventDetails {
            name: e.name,
            start_time: e.start_time.with_timezone(&Los_Angeles),
            end_time: e.end_time.map(|t| t.with_timezone(&Los_Angeles)),
            description: e.description,
            location: e.metadata.unwrap().location,
            rsvps: e.user_count.unwrap_or(0),
            discord_link: format!("https://discord.com/events/{}/{}", e.guild_id, e.id),
        }
    }
}

/// Handles discord API events related to discord guild events.
pub struct GuildEventHandler {
    pub db_pool: Pool<Sqlite>,
}

#[serenity::async_trait]
impl EventHandler for GuildEventHandler {
    async fn guild_scheduled_event_create(&self, ctx: Context, event: ScheduledEvent) {
        println!("Guild scheduled event create");

        // sqlx::query!(r#"
        //     INSERT INTO Events (eid, name, start_time, end_time, description, location, rsvps, discord_link)
        //     VALUES (
        //         ?,
        //         ?,
        //         ?,

        // "#)
    }

    async fn guild_scheduled_event_update(&self, ctx: Context, event: ScheduledEvent) {
        println!("Guild scheduled event update");
    }

    async fn guild_scheduled_event_delete(&self, ctx: Context, event: ScheduledEvent) {
        println!("Guild scheduled event delete");
    }

    async fn guild_scheduled_event_user_add(
        &self,
        ctx: Context,
        event: GuildScheduledEventUserAddEvent,
    ) {
        println!("Guild scheduled event user add");
    }

    async fn guild_scheduled_event_user_remove(
        &self,
        ctx: Context,
        event: GuildScheduledEventUserRemoveEvent,
    ) {
        println!("Guild scheduled event user add");
    }
}
