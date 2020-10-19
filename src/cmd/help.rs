// extern crate serenity;

// use std::collections::HashSet;

use serenity::{
    // framework::{
    //     StandardFramework,
    //     standard::{
    //         Args, HelpOptions, help_commands,
    //     },
    // },
    prelude::*,
    // http,
    model::prelude::*,
    model::id::ChannelId,
    framework::standard::{
        Args, CommandResult, CommandGroup, HelpOptions, StandardFramework,
        macros::{command},
    },
    utils::MessageBuilder,
};
// use serenity::framework::standard::help_commands::with_embeds as embed;

// #[help]
// #[individual_command_tip =
// "pass command as argument to learn more"]
// #[command_not_found_text = "couldn't find `{}`."]
#[command]
#[aliases("halp")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("command help:");
        m.tts(false);

        m.embed(|mut e| {
            e.title("known commands");
            e.description("
                prefix = `+`\n
                bean donut\n
                smug nia\n
                cringe\n
                the sex\n
                \n
                +child
                +ping
                +beans
                +smug
                +disgruntled
                +help
            ");
            // e.color()

            e
        });

        m
    }).await;

    Ok(())
}