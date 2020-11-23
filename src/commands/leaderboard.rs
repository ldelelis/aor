use std::collections::HashMap;
use std::iter;

use reqwest::Client;
use reqwest::header::COOKIE;

use serde::Deserialize;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command
};

#[derive(Debug, Deserialize)]
struct StarCompletion {
    pub get_star_ts: String
}

#[derive(Debug, Deserialize)]
struct Member {
    pub global_score: i32,
    pub stars: usize,
    pub local_score: i32,

    pub id: String,
    pub name: String,
    pub last_star_ts: String,

    pub completion_day_level: HashMap<String, HashMap<String, StarCompletion>>
}

#[derive(Debug, Deserialize)]
struct Leaderboard {
    pub members: HashMap<String, Member>,
    pub event: String,
    pub owner_id: String
}

#[command]
pub async fn leaderboard(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let url: String = format!("{}/leaderboard/private/view/{}.json", *crate::BASE_URL, env!("LEADERBOARD_ID"));
    let mut output = String::new();

    let client = Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let session_cookie: String = format!("session={}", env!("SESSION_COOKIE"));

    let response: Leaderboard = client.get(url.as_str())
        .header(COOKIE, session_cookie)
        .send()
        .await?
        .json()
        .await?;

    // TODO: format into an actual table lul

    for member in response.members.values() {
        output.push_str(format!("{}: {}\n", member.name, iter::repeat('*').take(member.stars).collect::<String>()).as_str());
    }

    msg.channel_id.say(&ctx.http, format!("```{}```", output)).await?;

    Ok(())
}
