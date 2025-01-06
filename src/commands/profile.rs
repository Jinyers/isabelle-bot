use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::{command, CreateReply};
use tracing::error;

fn create_embeded_profile(user: &serenity::User) -> CreateReply {
    let embed = serenity::CreateEmbed::new()
        .title("NookLink")
        .description(format!("Пользователь {}", serenity::Mention::from(user.id)));
    CreateReply::default().embed(embed)
}

#[command(slash_command, ephemeral)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "Lookup user's profile"]
    user: Option<serenity::User>
    ) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let result = ctx.send(create_embeded_profile(user)).await;
    if let Err(why) = result {
        error!("Cannot respond {} because {}", user.name, why);
    }
    Ok(())
}

#[command(context_menu_command = "Lookup profile", ephemeral)]
pub async fn profile_context_menu(
    ctx: Context<'_>,
    user: serenity::User
    ) -> Result<(), Error> {
    let result = ctx.send(create_embeded_profile(&user)).await;
    if let Err(why) = result {
        error!("Cannot respond {} because {}", user.name, why);
    }
    Ok(())
}
