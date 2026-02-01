use diesel::RunQueryDsl;
use serenity::all::{
    Context, EventHandler, GuildScheduledEventUserAddEvent, GuildScheduledEventUserRemoveEvent,
    ScheduledEvent,
};

use diesel::prelude::*;

use crate::database::connect_to_database;
use crate::database::models;
use crate::database::schema;

/// Handles discord API events related to discord guild events.
pub struct GuildEventHandler;

#[serenity::async_trait]
impl EventHandler for GuildEventHandler {
    async fn guild_scheduled_event_create(&self, _ctx: Context, discord_event: ScheduledEvent) {
        use schema::events::dsl::*;

        println!("Guild scheduled event create");

        let mut conn = connect_to_database();

        let event = models::Event::from(discord_event);

        println!("Event to be inserted: {:#?}", event);

        diesel::insert_into(events)
            .values(&event)
            .execute(&mut conn)
            .unwrap();
    }

    async fn guild_scheduled_event_update(&self, _ctx: Context, discord_event: ScheduledEvent) {
        use schema::events::dsl::*;

        println!("Guild scheduled event update");

        let mut conn = connect_to_database();

        let event = models::Event::from(discord_event);

        diesel::update(events.find(event.event_id))
            .set(&event)
            .execute(&mut conn)
            .unwrap();
    }

    async fn guild_scheduled_event_delete(&self, _ctx: Context, _event: ScheduledEvent) {
        println!("Guild scheduled event delete");
    }

    async fn guild_scheduled_event_user_add(
        &self,
        _ctx: Context,
        event: GuildScheduledEventUserAddEvent,
    ) {
        use schema::events::dsl::*;

        println!("Guild scheduled event user add");

        let id = event.scheduled_event_id.get() as f64;

        let mut conn = connect_to_database();

        // Increment the number of rsvps on the event with event_id=id
        diesel::update(events.find(id))
            .set(rsvps.eq(rsvps + 1))
            .execute(&mut conn)
            .unwrap();
    }

    async fn guild_scheduled_event_user_remove(
        &self,
        _ctx: Context,
        _event: GuildScheduledEventUserRemoveEvent,
    ) {
        println!("Guild scheduled event user remove");
    }
}
