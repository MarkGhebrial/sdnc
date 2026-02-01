use super::schema::events;
use chrono_tz::America;
use diesel::{
    Queryable, Selectable,
    prelude::{AsChangeset, Insertable},
};
use serenity::all::ScheduledEvent;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = events)]
pub struct Event {
    pub event_id: f64,
    pub guild_id: f64,
    pub start_time: String,
    pub end_time: Option<String>,
    pub event_name: String,
    pub event_description: Option<String>,
    pub event_location: Option<String>,
    pub rsvps: i32,
}

impl From<ScheduledEvent> for Event {
    fn from(value: ScheduledEvent) -> Self {
        Self {
            event_id: value.id.get() as f64,
            guild_id: value.guild_id.get() as f64,
            start_time: value
                .start_time
                .with_timezone(&America::Los_Angeles)
                .to_rfc3339(),
            end_time: value
                .end_time
                .map(|t| t.with_timezone(&America::Los_Angeles).to_rfc3339()),
            event_name: value.name,
            event_description: value.description,
            event_location: value.metadata.and_then(|m| m.location),
            rsvps: value.user_count.unwrap_or(0) as i32,
        }
    }
}
