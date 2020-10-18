use std::env;
// use std::io;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*
};

mod urls;
// use crate::urls::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "cringe" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::CRINGE).await {
                println!("Err sending message: {:?}", why);
            }
        } else if msg.content == "bean donut" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::BEAN_DONUT).await {
                println!("Err sending message: {:?}", why);
            }
        } else if msg.content == "smug nia" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::SMUG_NIA).await {
                println!("Err sending message: {:?}", why);
            }
        } else if msg.content == "!sex" || msg.content == "the sex" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::THE_SEX).await {
                println!("Err sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and ready", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("unable to find token via env vars");

    let mut client = Client::new(&token)
                        .event_handler(Handler)
                        .await
                        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client Error: {:?}", why);
    }
}
