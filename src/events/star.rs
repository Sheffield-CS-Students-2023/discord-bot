use serenity::prelude::*;
use serenity::async_trait;
use serenity::builder::EditMessage;
use serenity::all::{Message, Reaction, ReactionType, CreateEmbed, CreateEmbedAuthor, ChannelId};
use crate::structures::starboard::Starboard;

const MIN_STARS: usize = 3;
const STARBOARD_CHANNEL_ID: u64 = 1201172163613433987;

pub struct MongoClient;

impl TypeMapKey for MongoClient {
    type Value = mongodb::Client;
}

pub struct StarHandler;

fn make_starboard_embed(message: &Message) -> CreateEmbed {
    let reply = message.referenced_message.as_ref();
    let reply_indicator = if let Some(reply) = reply {
        format!(
            "<:reply:1176214702754377868> replying to [{}]({}): {}\n",
            reply.author.name,
            reply.link(),
            reply.content.chars().take(20).collect::<String>()
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
        .title(format!("{} 🌟", message.reactions.iter().filter(|r| r.reaction_type == ReactionType::Unicode("⭐".to_string())).count()))
        .author(CreateEmbedAuthor::new(&message.author.name).icon_url(&message.author.avatar_url().unwrap_or_default()))
        .color(0xFFD700)
        .description(format!("{}{}{}\n\n[Jump to message]({})", reply_indicator, message.content, attachment_name, message.link()))
        .timestamp(&message.timestamp);

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
        if reaction.emoji != ReactionType::Unicode("⭐".to_string()) {
            return;
        }
        let channel = ChannelId::new(STARBOARD_CHANNEL_ID);

        let mut data = ctx.data.write().await;
        let client = data.get_mut::<MongoClient>().unwrap();

        let starboard = Starboard::new(client).await;

        let reaction_channel = match reaction.channel_id.to_channel(&ctx).await {
            Ok(channel) => channel,
            Err(_) => return,
        };
        let reaction_message = match reaction_channel.id().message(&ctx.http, reaction.message_id).await {
            Ok(message) => message,
            Err(_) => return,
        };

        // Add a star to the starboard
        let data = starboard.add_star(
            reaction.message_id.into(), 
            reaction.user_id.unwrap().into(), 
            reaction_message.author.id.into()
        ).await;

        // If the message has enough stars, send it to the starboard
        if data.stars.len() == MIN_STARS && data.starboard_id.is_none() { // Message has just reached starboard threshold
            let embed = make_starboard_embed(&reaction_message);

            let mut message = reaction_channel.id().say(&ctx.http, &reaction_message.channel_id.mention().to_string()).await.unwrap();
            message.edit(&ctx.http, EditMessage::new().embed(embed)).await.unwrap();
            starboard.update_starboard_message(data.id, message.id.into()).await;
            channel.say(&ctx.http, embed).await.unwrap();
        } else if data.stars.len() >= MIN_STARS {
            // Edit the starboard message to reflect the new amount of stars
            let mut message = reaction_channel.id().message(&ctx.http, data.starboard_id.unwrap() as u64).await.unwrap();
            let mut embed = message.embeds.first().unwrap().clone();
            embed.title = Some(format!("{} 🌟", data.stars.len()));
            message.edit(&ctx.http, EditMessage::new().embed(embed.into())).await.unwrap();
        }
    } 

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if reaction.emoji != ReactionType::Unicode("⭐".to_string()) {
            return;
        }

        let channel = ChannelId::new(STARBOARD_CHANNEL_ID);

        let mut data = ctx.data.write().await;
        let client = data.get_mut::<MongoClient>().unwrap();

        let starboard = Starboard::new(client).await;

        // Remove a star from the starboard
        let data = starboard.remove_star(
            reaction.message_id.into(), 
            reaction.user_id.unwrap().into()
        ).await;

        if data.is_none() {
            return;
        }

        if data.as_ref().unwrap().starboard_id.is_none() {
            return; // Not on the starboard
        }

        // Edit the starboard message to reflect the new amount of stars
        let mut message = channel.message(&ctx.http, data.clone().unwrap().starboard_id.unwrap() as u64).await.unwrap();
        let mut embed = message.embeds.first().unwrap().clone();
        embed.title = Some(format!("{} 🌟", data.unwrap().stars.len()));
        message.edit(&ctx.http, EditMessage::new().embed(embed.into())).await.unwrap();

    }
        
}