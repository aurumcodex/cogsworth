use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};

use tracing::info;

use crate::urls::*;

#[command]
async fn beans(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `beans` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &BEANS).await?;

    Ok(())
}

#[command]
async fn disgruntled(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `disgruntled` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &DISGRUNTLED).await?;

    Ok(())
}

#[command]
async fn child(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `child` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &CHILD).await?;

    Ok(())
}

#[command]
async fn smug(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `smug` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &SMUG_NIA).await?;

    Ok(())
}

#[command]
async fn rat(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &RAT).await?;

    Ok(())
}

#[command]
async fn rat_chair(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &RAT_CHAIR).await?;

    Ok(())
}

#[command]
async fn mock(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &JACK_FROST2).await?;

    Ok(())
}

#[command]
async fn death(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &JACK_FROST1).await?;

    Ok(())
}

#[command]
async fn hee_homeboys(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &JACK_FROST3).await?;

    Ok(())
}

#[command]
async fn wot(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &WOT_NIA).await?;

    Ok(())
}

#[command]
async fn bread(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &BREAD).await?;

    Ok(())
}

#[command]
async fn hydrate(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &HYDRATE).await?;

    Ok(())
}

#[command]
async fn panic(ctx: &Context, msg: &Message) -> CommandResult {
    info!("got `rat_chair` command from {} in channel {}", msg.author.name, msg.channel_id);
    msg.channel_id.say(&ctx.http, &PANIC).await?;

    Ok(())
}