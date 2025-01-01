use std::env;
use tracing::{
    info,
    error,
};
use tracing_subscriber;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, event::ResumedEvent},
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("New message");
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                error!("Error sending message: {why:?}");
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
    async fn resume(&self, _:Context, _: ResumedEvent) {
        info!("Resumed")
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents)
            .event_handler(Handler).await
            .expect("Err creating client");
    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }

}
