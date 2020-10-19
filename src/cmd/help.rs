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
    // model::id::ChannelId,
    framework::standard::{
        Args, CommandResult, CommandGroup, HelpOptions, StandardFramework,
        macros::{command},
    },
    // colour,
    utils::{/*MessageBuilder,*/ Colour},
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

        m.embed(|e| {
            e.title("known commands");
            e.description("
                prefix = `none`\n
                bean donut
                smug nia
                cringe
                the sex
                \n
                prefix = `+`\n
                +child
                +ping
                +beans
                +smug
                +disgruntled
                +help
            ");
            e.color(Colour::from_rgb(157, 192, 49));
            // e.color()

            e
        });

        m
    }).await;

    Ok(())
}