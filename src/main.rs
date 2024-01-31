extern crate dotenv;

mod events;
mod structs;

use events::*;
use dotenv::dotenv;
use structs::{
    Data,
    Error
};
use std::env::var;
use poise::{
    FrameworkOptions,
    serenity_prelude::{
        ClientBuilder,
        GatewayIntents
    }
};

#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;



#[tokio::main]
async fn main(){
    dotenv().ok();
    
    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var");
        
    let intents =
        GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    guild_id: var("GUILD_ID").expect("guild_id").parse::<u64>().expect("u64 guild_id"),
                    voice_id: var("GUILD_VOICE_ID").expect("voice_id").parse::<u64>().expect("u64 voice_id")
                })
            })
        })
        .options(FrameworkOptions {
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler(ctx, event, framework))
            },
            
            ..Default::default()
        })
        .build();

    let client = ClientBuilder::new(token, intents) 
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}