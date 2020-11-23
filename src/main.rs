mod commands;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::Permissions;
use serenity::model::gateway::Ready;
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};

use commands::{
    leaderboard::*,
    repos::*,
    input::*
};

// FIXME: this is probably wrong, and bad, and ugly. research
// floppy says to use once_cell
lazy_static::lazy_static! {
    static ref BASE_URL: String = format!(
        "https://adventofcode.com/{}",
        env!("YEAR")
    );
}

#[group]
#[prefix = "repos"]
#[commands(repos, add_repo)]
struct Repos;

#[group]
#[commands(leaderboard, input)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let url = match ready.user.invite_url(&ctx, Permissions::empty()).await {
            Ok(v) => v,
            Err(exc) => {
                println!("error getting inv: {:?}", exc);

                return
            }
        };
        println!("{}", url);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("|>"))
        .group(&GENERAL_GROUP)
        .group(&REPOS_GROUP);

    let token = env!("TOKEN");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(exc) = client.start().await {
        println!("something borked: {:?}", exc);
    }
}
