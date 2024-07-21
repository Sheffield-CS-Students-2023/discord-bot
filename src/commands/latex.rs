use crate::{Context, Error};
use poise::command;
use urlencoding::encode;

#[command(prefix_command)]
pub async fn latex(ctx: Context<'_>, args: String) -> Result<(), Error> {
    ctx.say(format!(
        "https://latex.codecogs.com/png.latex?\\dpi{{250}}\\bg{{white}}{}",
        encode(&args)
    ))
    .await?;

    Ok(())
}
