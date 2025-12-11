use serenity::all::{
    Context, EventHandler, GuildScheduledEventUserAddEvent, GuildScheduledEventUserRemoveEvent,
    ScheduledEvent,
};

/// Handles discord API events related to discord guild events.
pub struct GuildEventHandler;

#[serenity::async_trait]
impl EventHandler for GuildEventHandler {
    async fn guild_scheduled_event_create(&self, ctx: Context, event: ScheduledEvent) {
        println!("Guild scheduled event create");
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
