use std::env;

use regex::Regex;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let re = Regex::new(r"[Dd][Ee][Uu][Ss] [Vv][Uu][Ll][Tt]").unwrap();
        if msg.author.id != 840678965114306590 {
            if re.is_match(&msg.content) {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Deus vult ferrūginem!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
