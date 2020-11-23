use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write, Seek, SeekFrom};

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command
};

static REPO_DB: &str = "repos.txt";

#[command("list")]
pub async fn repos(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let mut data = String::new();

    let mut f = OpenOptions::new()
        .read(true)
        .open(REPO_DB)
        .unwrap();
    f.read_to_string(&mut data).expect("cant read my dude");

    if data.is_empty() {
        msg.channel_id.say(&ctx.http, "No repos registered yet").await?;
        return Ok(());
    }

    msg.channel_id.say(&ctx.http, data).await?;

    Ok(())
}

#[command("add")]
pub async fn add_repo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let repo_url: String = args.single().unwrap();
    let user_tag = msg.author.tag();

    let mut data = String::new();
    let mut tmp_db: HashMap<String, String> = HashMap::new();

    let mut f = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(REPO_DB)
        .unwrap();
    f.read_to_string(&mut data).expect("failed to read for some reason");
    f.seek(SeekFrom::Start(0)).unwrap();

    data.split('\n')
        .for_each(|x| {
            if !x.is_empty() {
                tmp_db.insert(
                    x.split('=').collect::<Vec<_>>()[0].to_string(),
                    x.split('=').collect::<Vec<_>>()[1].to_string()
                );
            }
            ()
        });

    let action: &str = if tmp_db.contains_key(&user_tag) {"Updated"} else {"Added"};

    tmp_db.insert(user_tag, repo_url);

    f.write_all(
        tmp_db.iter()
            .map(|(k, v)| format!("{}={}\n", k, v))
            .collect::<Vec<_>>()
            .concat()
            .as_bytes()
    ).unwrap();

    msg.channel_id.say(&ctx.http, format!("{} your repo successfully", action)).await?;

    Ok(())
}
