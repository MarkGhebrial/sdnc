use serenity::{
    all::{
        Context, EventHandler, GuildScheduledEventUserAddEvent, GuildScheduledEventUserRemoveEvent,
        ScheduledEvent,
    },
};

use crate::database::connect_to_database;

/// Handles discord API events related to discord guild events.
pub struct GuildEventHandler;

#[serenity::async_trait]
impl EventHandler for GuildEventHandler {
    async fn guild_scheduled_event_create(&self, _ctx: Context, event: ScheduledEvent) {
        println!("Guild scheduled event create");

        let connection = connect_to_database().unwrap();

        connection.execute(
            "INSERT INTO Events (EventID, GuildID, EventName, StartTime, EndTime, EventDescription, EventLocation, Rsvps) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);"
        , (
            event.id.get() as i64,
            event.guild_id.get() as i64,
            event.name,
            event.start_time.with_timezone(&chrono_tz::America::Los_Angeles).to_rfc3339(), // TODO: Sqlite uses rfc 8601, not rfc 3339
            event.end_time.map(|e| e.with_timezone(&chrono_tz::America::Los_Angeles).to_rfc3339()),
            event.description,
            event.metadata.map(|m| m.location).unwrap_or_default(),
            event.user_count.map(|c| c as i64).unwrap_or(0),
        )).unwrap();
    }

    async fn guild_scheduled_event_update(&self, _ctx: Context, _event: ScheduledEvent) {
        println!("Guild scheduled event update");
    }

    async fn guild_scheduled_event_delete(&self, _ctx: Context, _event: ScheduledEvent) {
        println!("Guild scheduled event delete");
    }

    async fn guild_scheduled_event_user_add(
        &self,
        _ctx: Context,
        _event: GuildScheduledEventUserAddEvent,
    ) {
        println!("Guild scheduled event user add");
    }

    async fn guild_scheduled_event_user_remove(
        &self,
        _ctx: Context,
        _event: GuildScheduledEventUserRemoveEvent,
    ) {
        println!("Guild scheduled event user add");
    }
}
