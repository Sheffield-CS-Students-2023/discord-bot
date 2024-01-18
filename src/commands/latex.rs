use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use urlencoding::encode;

#[command]
pub async fn latex(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let expression = args.single::<String>()?;
    msg.channel_id.say(
        &ctx.http, 
        format!(
            "https://latex.codecogs.com/png.latex?\\dpi{{250}}\\bg{{white}}{}", 
            encode(&expression)
        )
    ).await?;

    Ok(())
}