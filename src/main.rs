//! Cogsworth is a bot for a discord server.

mod cmd;
mod urls;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use rand::prelude::*;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::{group, hook, help},
    },
    http::Http,
    model::prelude::*,
    prelude::*,
};

use tracing::{debug, error, info, instrument};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};


use cmd::{
    owner::*,
    meta::*,
    images::*,
    help::*,
};

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "cringe" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::CRINGE).await {
                error!("error sending message: {:?}", why);
            }
        } else if msg.content == "bean donut" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::BEAN_DONUT).await {
                error!("error sending message: {:?}", why);
            }
        } else if msg.content == "the sex" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::THE_SEX).await {
                error!("error sending message: {:?}", why);
            }
        } else if msg.content == "hee ho" {
            let rng = rand::thread_rng().gen_range(0, 3);
            match rng {
                0 => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, urls::JACK_FROST1).await {
                        error!("error sending message: {:?}", why);
                    }
                },
                1 => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, urls::JACK_FROST2).await {
                        error!("error sending message: {:?}", why);
                    }
                },
                2 => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, urls::JACK_FROST3).await {
                        error!("error sending message: {:?}", why);
                    }
                },
                _ => { info!("unable to send jack frost image"); }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and ready", ready.user.name);
        info!("connected as: {}", ready.user.name);
    }

    #[instrument(skip(self))]
    async fn resume(&self, _: Context, resume: ResumedEvent) {
        info!("Resumed; trace: {:?}", resume.trace);
    }
}

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, cmd_name: &str) -> bool {
    info!("Got command `{}` by user `{}` in channel `{}`", cmd_name, msg.author.name, msg.channel_id);

    true
}

#[group]
#[commands(quit, ping, help)]
struct General;

#[group]
#[commands(beans, disgruntled, child, smug, rat, rat_chair, wot, death, hee_homeboys, mock, bread, hydrate)]
struct Images;

#[tokio::main]
#[instrument]
async fn main() {
    dotenv::dotenv().expect("failed to read .env file");

    // let subscriber = FmtSubscriber::builder()
    //                     .with_env_filter(EnvFilter::from_default_env())
    //                     .finish();

    // tracing::subscriber::set_global_default(subscriber).expect("failed to start the logger");

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("unable to find token via env vars");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access applicaion info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix("+")
            .owners(owners)
        )
        .before(before)
        // .help(&HELP_FN)
        .group(&GENERAL_GROUP)
        .group(&IMAGES_GROUP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        error!("Client Error: {:?}", why);
    }
}
