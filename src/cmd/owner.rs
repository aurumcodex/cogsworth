use crate::ShardManagerContainer;

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "shutting down").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "there was a problem getting the shard manager").await?;

        return Ok(());
    }

    Ok(())
}