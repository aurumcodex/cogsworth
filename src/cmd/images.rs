use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

use tracing::info;

use crate::urls::{
    BEANS,
    DISGRUNTLED,
    SMUG_NIA,
    CHILD,

};

#[command]
async fn beans(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `beans` command from {}", msg.channel_id);
    msg.channel_id.say(&ctx.http, &BEANS).await?;

    Ok(())
}

#[command]
async fn disgruntled(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `disgruntled` command from {}", msg.channel_id);
    msg.channel_id.say(&ctx.http, &DISGRUNTLED).await?;

    Ok(())
}

#[command]
async fn child(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `child` command from {}", msg.channel_id);
    msg.channel_id.say(&ctx.http, &CHILD).await?;

    Ok(())
}

#[command]
async fn smug(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `smug` command from {}", msg.channel_id);
    msg.channel_id.say(&ctx.http, &SMUG_NIA).await?;

    Ok(())
}