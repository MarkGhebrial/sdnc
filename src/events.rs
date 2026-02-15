use std::sync::Arc;

use chrono::Utc;
use diesel::RunQueryDsl;
use serenity::all::{GuildId, ScheduledEventStatus};
use serenity::all::{
    Context, EventHandler, GuildScheduledEventUserAddEvent, GuildScheduledEventUserRemoveEvent,
    ScheduledEvent,
};

use diesel::prelude::*;

use crate::config::CONFIG;
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

        diesel::insert_into(events)
            .values(&event)
            .execute(&mut conn)
            .unwrap();
    }

    async fn guild_scheduled_event_update(&self, _ctx: Context, discord_event: ScheduledEvent) {
        use schema::events::dsl::*;

        println!("Guild scheduled event update");

        // Don't update the event if it's completed
        // TODO: This doesn't actually do what I need it to do
        if discord_event.status == ScheduledEventStatus::Completed {
            println!("Event already completed, skipping update");
            return;
        }

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
        discord_event: GuildScheduledEventUserAddEvent,
    ) {
        use schema::events::dsl::*;

        println!("Guild scheduled event user add");

        let id = discord_event.scheduled_event_id.get() as f64;

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
        discord_event: GuildScheduledEventUserRemoveEvent,
    ) {
        use schema::events::dsl::*;

        println!("Guild scheduled event user remove");

        let id = discord_event.scheduled_event_id.get() as f64;

        let mut conn = connect_to_database();

        // Decrement the number of rsvps on the event with event_id=id
        diesel::update(events.find(id))
            .set(rsvps.eq(rsvps - 1))
            .execute(&mut conn)
            .unwrap();
    }
}

/// Get the events currently posted to the discord server, then update the database.
///
/// This happens in a couple of steps:
/// 1. Get all the scheduled events from the discord guild
/// 2. For each event from the discord guild, check to see if it's already in the
///    database.
///    - If it is in the database, run an update query
///    - If it is not in the database, run an insert query
/// 3. For each *future* event in the database, check to see if that event is still
///    in the discord guild.
///    - If not, remove the event from the database
///
/// Run this function on a schedule to make sure the database of events does not
/// fall out of sync with the discord guild. (Falling out of sync happens if the
/// backend misses some updates from the discord API.)
pub async fn synchronize_events(http: Arc<serenity::all::Http>) {
    // Fetch the events that are currently on the discord guild
    let guild = GuildId::new(CONFIG.discord.guild_id);
    let discord_events: Vec<ScheduledEvent> = guild.scheduled_events(&http, true).await.unwrap();

    let mut conn = connect_to_database();

    use schema::events::dsl::*;

    // Update or insert out of date events
    for event in discord_events
        .iter()
        .map(|e| models::Event::from(e.clone()))
    {
        println!("Updating event: {:#?}", event);

        diesel::insert_into(events)
            .values(&event)
            .on_conflict(event_id)
            .do_update()
            .set(&event)
            .execute(&mut conn)
            .unwrap();
    }

    /**** Delete events that are no longer on discord ****/

    // For some reason, the query doesn't work properly unless we replace the "T"
    // separator with a space. Genuinely no idea why that is. Maybe some weird diesel
    // behavior?
    let current_time = Utc::now();//.to_rfc3339().replace("T", " ");

    let discord_event_ids: Vec<f64> = discord_events.iter().map(|e| e.id.get() as f64).collect();

    // Get the event ids of events that have not ended yet
    // SELECT event_id FROM events E WHERE E.end_time >= date();
    let event_ids: Vec<f64> = events
        .select(event_id)
        .filter(end_time.assume_not_null().gt(current_time))
        .load(&mut conn)
        .unwrap();

    // Set difference between the future events in the database and the events in
    // the discord guild
    let event_ids_to_remove: Vec<f64> = event_ids
        .into_iter()
        .filter(|id| !discord_event_ids.contains(id))
        .collect();

    for id in event_ids_to_remove {
        // DELETE events E WHERE E.event_id == {event_id_to_remove}
        diesel::delete(events)
            .filter(event_id.eq(id))
            .execute(&mut conn)
            .unwrap();
    }

    // let event_ids = event_ids.into_iter().filter(|id| !discord_event_ids.contains(id)).collect();
}
