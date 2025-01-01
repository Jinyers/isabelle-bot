use poise::serenity_prelude as serenity;
use crate::{Context, Error};

/// Basic ping command
///
/// Yeah, it's very simple
#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content("Pong!")
            .ephemeral(false), // this one only applies in application commands though
    )
    .await?;
    Ok(())
}
