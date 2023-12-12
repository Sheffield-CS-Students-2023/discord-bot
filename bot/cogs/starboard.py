import discord
from discord.ext import commands
import re

from bot.bot import Bot
from bot.utils.classes import Starboard as StarboardClass
from bot.static.constants import GUILD_ID, MIN_STARS, STARBOARD_CHANNEL_ID

class Starboard(commands.Cog):

    def __init__(self, client: Bot):
        self.client = client

    @property
    def guild(self) -> discord.Guild:
        return self.client.get_guild(GUILD_ID)

    @property
    def channel(self) -> discord.TextChannel:
        return self.guild.get_channel(STARBOARD_CHANNEL_ID)
    
    def make_starboard_embed(self, message: discord.Message) -> discord.Embed:
        reply = message.reference.resolved if message.reference else None
        # If there is a reply and the content of the mesage replied to has a link within the first 20 characters, replace the actual link with "Link"
        if reply and reply.content and reply.content[:20].find("http") != -1:
            # Remove the enire link and replace it wuth "Link" using regex
            reply.content = re.sub(r"http[a-zA-Z0-9._/:]", "Link", reply.content)

        reply_indicator = (f"<:reply:1176214702754377868> replying to [{reply.author.display_name}]({message.reference.jump_url}): {reply.content[:20]}{'...' if len(reply.content) > 20 else ''}\n" if reply else "")

        unembeddable_attachment = message.attachments and message.attachments[0].content_type and not message.attachments[0].content_type.startswith("image")
        attachment_name = f"\n[{message.attachments[0].filename}]({message.attachments[0].url})" if unembeddable_attachment else ""
    
        embed = discord.Embed.from_dict(
            {
                "title": str([r for r in message.reactions if str(r) == "\U00002b50"][0].count) + " 🌟", # This assumes at least one star is still present and will break if not
                "author": {
                    "name": message.author.display_name,
                    "icon_url": message.author.display_avatar.url if message.author.display_avatar else discord.DefaultAvatar.blurple
                },
                "color": int(discord.Colour.gold()),
                "description": reply_indicator + message.content + attachment_name + f"\n\n[Jump to message]({message.jump_url})",
                "timestamp": message.created_at.isoformat(),
            }
        )
        # Use regex to see if message contains a direct png, jpg or gif link

        attachment_link = re.search(r"(http[a-zA-Z0-9._/:]+(png|jpg|gif))", message.content)

        if message.attachments:
            embed.set_image(url=message.attachments[0].url)
        elif attachment_link:
            embed.set_image(url=attachment_link.group(0))
            embed.description = embed.description.replace(attachment_link.group(0), "")

        return embed
    

    @commands.Cog.listener("on_raw_reaction_add")
    async def on_reaction_add(self, payload: discord.RawReactionActionEvent):
        # Check if reaction is a star using unicode
        if str(payload.emoji) != "\U00002b50" or \
                payload.guild_id != GUILD_ID or \
                    payload.channel_id == STARBOARD_CHANNEL_ID:
            return
        
        starboard = StarboardClass()

        if payload.message_id in starboard and payload.user_id in starboard[payload.message_id]: # Edgecase
            return

        reaction_channel = await self.client.fetch_channel(payload.channel_id)
        reaction_message = await reaction_channel.fetch_message(payload.message_id)

        # Add a star to the starboard
        data = starboard.add_star(payload.message_id, payload.user_id, reaction_message.author.id)

        # If the message has enough stars, send it to the starboard
        if len(data["stars"]) == MIN_STARS and not data["starboard_id"]: # Message has just reached starboard threshold
            embed = self.make_starboard_embed(reaction_message)

            message = await self.channel.send(content=reaction_message.channel.mention, embed=embed)
            _data = starboard[reaction_message.id]
            _data["starboard_id"] = message.id
            starboard[reaction_message.id] = _data

        elif len(data["stars"]) >= MIN_STARS:
            # Edit the starboard message to reflect the new amount of stars
            message = await self.channel.fetch_message(data["starboard_id"])
            embed = message.embeds[0]
            embed.title = str(len(data["stars"])) + " 🌟"
            await message.edit(embed=embed)

    @commands.Cog.listener("on_raw_reaction_remove")
    async def on_reaction_remove(self, payload: discord.RawReactionActionEvent):
        if str(payload.emoji) != "\U00002b50" or \
                payload.guild_id != GUILD_ID or \
                    payload.channel_id == STARBOARD_CHANNEL_ID:
            return
        
        starboard = StarboardClass()

        # reaction_channel = await self.client.fetch_channel(payload.channel_id)
        # reaction_message = await reaction_channel.fetch_message(payload.message_id)

        # Remove a star from the starboard
        data = starboard.remove_star(payload.message_id, payload.user_id)

        if not data:
            return
    
        # If the message count has fallen below the threshold, delete it from the starboard
        # if len(data["stars"]) < MIN_STARS and data["starboard_id"]:
        #     message = await self.channel.fetch_message(data["starboard_id"])
        #     await message.delete()
        #     starboard[reaction_message.id]["starboard_id"] = None
        if data["starboard_id"]: # If a star is removed from a message with 3+ stars starboard has not tracked
            # Edit the starboard message to reflect the new amount of stars
            message = await self.channel.fetch_message(data["starboard_id"])
            embed = message.embeds[0]
            embed.title = str(len(data["stars"])) + " 🌟"
            await message.edit(embed=embed)

    @discord.app_commands.command()
    async def randomstar(self, interaction: discord.Interaction):
        """Get a random starboard message"""
        starboard = StarboardClass()
        if not starboard:
            return await interaction.response.send_message("No starboard messages found", ephemeral=True)

        message = await self.channel.fetch_message(starboard.random_star_message_id)
        await interaction.response.send_message(content=message.content, embed=message.embeds[0], ephemeral=True)

Cog = Starboard