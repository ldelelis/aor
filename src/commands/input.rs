use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command
};

#[command]
pub async fn input(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let input_url: String = format!("{}/day/{}/input", *crate::BASE_URL, args.single::<i8>()?);
    msg.channel_id.say(&ctx.http, input_url).await?;

    Ok(())
}
