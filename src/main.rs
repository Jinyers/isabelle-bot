mod commands;

use std::env;
use tracing::error;
use tracing_subscriber;

use serenity::prelude::*;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token");
    let commands = vec![
        commands::ping::ping(),
        commands::profile::profile(),
        commands::profile::profile_context_menu(),
    ];

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {})
        })
    })
    .build();

    let mut client =
        Client::builder(&token, intents)
            .framework(framework)
            .await
            .expect("Err creating client");
    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }

}
