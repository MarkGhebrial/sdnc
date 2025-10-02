use std::env;

use serenity::all::{Guild, Shard};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

const GUILD_ID: u64 = 1224949123141210173;

struct Handler;

async fn generate_invite(client: Client) {
    let guild = serenity::model::id::GuildId::new(GUILD_ID);

    // guild.invites(client.http).await.unwrap()

    todo!();
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_SCHEDULED_EVENTS | GatewayIntents::GUILD_INVITES;

    let client_builder = Client::builder(&token, intents);

    // let application_id = client_builder.get_application_id().unwrap();
    // application_id

    let client = client_builder.await.unwrap();

    let guild = serenity::model::id::GuildId::new(GUILD_ID);

    // let server_name = guild.name(client.cache).unwrap();
    // println!("Server name is '{server_name}'");

    let events = guild.scheduled_events(client.http, true).await.unwrap();

    for event in events {
        println!("Event name: {}", event.name);
        println!("Event time: {} - {:?}", event.start_time, event.end_time);
        println!("Event location: {:?}", event.metadata.unwrap().location);
        println!("Number of RSVPs: {:?}", event.user_count);

        println!("Description: {:?}", event.description)
        // println!("{event:?}");
    }

    // client.
}

struct EventDetails {
    name: Option<String>,
    start_time: (),
    end_time: (),
    location: Option<String>,
    rsvps: usize,
}
