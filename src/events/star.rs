use crate::structures::starboard::Starboard;
use serenity::all::{
    ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, EditMessage, Message, Reaction,
    ReactionType,
};
use serenity::async_trait;
use serenity::prelude::*;

const MIN_STARS: usize = 3;
pub const STARBOARD_CHANNEL_ID: u64 = 1162423699455090748;

pub struct MongoClient;

impl TypeMapKey for MongoClient {
    type Value = mongodb::Client;
}

pub struct StarHandler;

static EMOJIS: [&str; 2] = ["⭐", "💀"];

fn make_starboard_embed(message: &Message, emoji: &str) -> CreateEmbed {
    let reply = message.referenced_message.as_ref();
    let reply_indicator = if let Some(reply) = reply {
        format!(
            "<:reply:1176214702754377868> replying to [{}]({}): {}\n",
            reply.author.name,
            reply.link(),
            reply.content.chars().take(100).collect::<String>()
        )
    } else {
        String::new()
    };

    let attachment_name = if let Some(attachment) = message.attachments.first() {
        format!("\n[{}]({})", attachment.filename, attachment.url)
    } else {
        String::new()
    };

    let mut embed = CreateEmbed::default()
        .title(format!(
            "{} {}",
            MIN_STARS,
            emoji
        ))
        .author(
            CreateEmbedAuthor::new(&message.author.name)
                .icon_url(message.author.avatar_url().unwrap_or_default()),
        )
        .color(0xFFD700)
        .description(format!(
            "{}{}{}\n\n[Jump to message]({})",
            reply_indicator,
            message.content,
            attachment_name,
            message.link()
        ))
        .timestamp(message.timestamp);

    if let Some(attachment) = message.attachments.first() {
        embed = embed.image(attachment.url.as_str());
    } else if let Some(attachment_link) = message.content.find("http") {
        embed = embed.image(&message.content[attachment_link..]);
    }

    embed
}

#[async_trait]
impl EventHandler for StarHandler {
    // When a message is reacted to

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // Return if emoji isn't in valid emojis
        if !EMOJIS.iter().any(|&emoji| reaction.emoji == ReactionType::Unicode(emoji.to_string())) {
            return;
        }

        // Hardcode emoji choices for now
        let emoji = if reaction.emoji == ReactionType::Unicode("⭐".to_string()) { "🌟" } else { "💀" }; 

        if reaction.channel_id == STARBOARD_CHANNEL_ID {
            return;
        }

        let channel = ChannelId::new(STARBOARD_CHANNEL_ID);

        let mut data = ctx.data.write().await;
        let client = data.get_mut::<MongoClient>().unwrap();

        let starboard = Starboard::new(client).await;

        let Ok(reaction_channel) = reaction.channel_id.to_channel(&ctx).await else {
            return;
        };

        let Ok(reaction_message) = reaction_channel
            .id()
            .message(&ctx.http, reaction.message_id)
            .await
        else {
            return;
        };

        // Add a star to the starboard
        let data = starboard
            .add_star(
                reaction.message_id.into(),
                reaction.user_id.unwrap().into(),
                reaction_message.author.id.into(),
            )
            .await;

        // If the message has enough stars, send it to the starboard
        if data.stars.len() == MIN_STARS && data.starboard_id.is_none() {
            // Message has just reached starboard threshold
            let embed = make_starboard_embed(&reaction_message, emoji);

            let message = channel
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content(&reaction_message.channel_id.mention().to_string())
                        .embed(embed),
                )
                .await
                .unwrap();
            starboard
                .update_starboard_message(data._id, message.id.into())
                .await;
            // channel.say(&ctx.http, embed).await.unwrap();
        } else if data.stars.len() >= MIN_STARS {
            // Edit the starboard message to reflect the new amount of stars
            let mut message = channel
                .message(&ctx.http, data.starboard_id.unwrap() as u64)
                .await
                .unwrap();
            let mut embed = message.embeds.first().unwrap().clone();         
            let mut stars = data.stars.clone();
            stars.sort();
            stars.dedup();
            embed.title = Some(format!("{} {}", stars.len(), emoji));
            message
                .edit(&ctx.http, EditMessage::new().embed(embed.into()))
                .await
                .unwrap();
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        // Return if emoji isn't in valid emojis
        if !EMOJIS.iter().any(|&emoji| reaction.emoji == ReactionType::Unicode(emoji.to_string())) {
            return;
        }

        let channel = ChannelId::new(STARBOARD_CHANNEL_ID);

        let mut data = ctx.data.write().await;
        let client = data.get_mut::<MongoClient>().unwrap();

        let starboard = Starboard::new(client).await;

        // Remove a star from the starboard
        let data = starboard
            .remove_star(reaction.message_id.into(), reaction.user_id.unwrap().into())
            .await;

        if data.is_none() {
            return;
        }

        if data.as_ref().unwrap().starboard_id.is_none() {
            return; // Not on the starboard
        }

        // Edit the starboard message to reflect the new amount of stars
        let mut message = channel
            .message(
                &ctx.http,
                data.clone().unwrap().starboard_id.unwrap() as u64,
            )
            .await
            .unwrap();
        let mut embed = message.embeds.first().unwrap().clone();
        let emoji = if reaction.emoji == ReactionType::Unicode("⭐".to_string()) { "🌟" } else { "💀" };          
        let mut stars = data.unwrap().stars.clone();
        stars.sort();
        stars.dedup();
        embed.title = Some(format!("{} {}", stars.len(), emoji));
        message
            .edit(&ctx.http, EditMessage::new().embed(embed.into()))
            .await
            .unwrap();
    }
}
