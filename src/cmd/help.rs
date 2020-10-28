// extern crate serenity;

use std::collections::HashSet;

use serenity::{
    framework::{
        StandardFramework,
        standard::{
            Args, HelpOptions, help_commands,
        },
    },
    prelude::*,
    // http,
    model::prelude::*,
    // model::id::ChannelId,
    framework::standard::{
        /*Args,*/ CommandResult, CommandGroup, /*HelpOptions, StandardFramework,*/
        macros::{command, help},
    },
    // colour,
    utils::{/*MessageBuilder,*/ Colour},
};
// use serenity::framework::standard::help_commands::with_embeds as embed;

// #[help]
// #[individual_command_tip =
// "pass command as argument to learn more"]
// #[command_not_found_text = "couldn't find `{}`."]
// async fn help_fn(
//     ctx: &Context,
//     msg: &Message,
//     args: Args,
//     help_options: &'static HelpOptions,
//     groups: &[&'static CommandGroup],
//     owners: HashSet<UserId>
// ) -> CommandResult {
//     let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;

//     Ok(())
// }
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
                cringe
                hee ho
                the sex
                \n
                prefix = `+`\n
                +beans
                +bread
                +child
                +death
                +disgruntled
                +hee_homeboys
                +hydrate
                +mock
                +panic
                +ping
                +smug
                +rat
                +rat_chair
                +wot
                +help
            ");
            e.color(Colour::from_rgb(157, 192, 49));

            e
        });

        m
    }).await;

    Ok(())
}