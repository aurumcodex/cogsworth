//! Cogsworth is a bot for a discord server.

mod cmd;
mod urls;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::{group},
    },
    http::Http,
    model::prelude::*,
    prelude::*,
};

use tracing::{error, info};
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
        } else if msg.content == "smug nia" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::SMUG_NIA).await {
                error!("error sending message: {:?}", why);
            }
        } else if msg.content == "the sex" {
            if let Err(why) = msg.channel_id.say(&ctx.http, urls::THE_SEX).await {
                error!("error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and ready", ready.user.name);
        info!("connected as: {}", ready.user.name);
    }
}

#[group]
#[commands(quit, ping, help)]
struct General;

#[group]
#[commands(beans, disgruntled, child, smug)]
struct Images;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("failed to read .env file");

    let subscriber = FmtSubscriber::builder()
                        .with_env_filter(EnvFilter::from_default_env())
                        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("failed to start the logger");

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
        println!("Client Error: {:?}", why);
    }
}
