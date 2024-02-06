use crate::events::star::STARBOARD_CHANNEL_ID;
use crate::structures::starboard::Starboard;
use crate::MongoClient;
use crate::{Context, Error};
use poise::{command, CreateReply};
use serenity::builder::CreateEmbed;
use serenity::model::id::{ChannelId, MessageId};

#[command(slash_command, ephemeral)]
pub async fn randomstar(ctx: Context<'_>) -> Result<(), Error> {
    let mut data = ctx.serenity_context().data.write().await;
    let client = data.get_mut::<MongoClient>().unwrap();

    let starboard = Starboard::new(client).await;

    let star_id = starboard.get_random_star_message_id().await;

    if star_id.is_none() {
        ctx.say("No stars found").await?;
        return Ok(());
    }

    let star_id = star_id.unwrap();

    // Get message from starbaord channel with the star_id

    let channel = ChannelId::new(STARBOARD_CHANNEL_ID);

    let starboard_message = channel
        .message(&ctx.http(), MessageId::from(star_id as u64))
        .await;

    if starboard_message.is_err() {
        println!("Uh oh! STINKY!!");
        ctx.say("No stars found").await?;
        return Ok(());
    }

    let starboard_message = starboard_message.unwrap();

    // Copy content and embed of starboard_message and reply with it
    let content = starboard_message.content.clone();
    let embed = starboard_message.embeds.first().unwrap().clone();

    ctx.send(
        CreateReply::default()
            .content(content)
            .embed(CreateEmbed::from(embed)),
    )
    .await?;

    Ok(())
}
