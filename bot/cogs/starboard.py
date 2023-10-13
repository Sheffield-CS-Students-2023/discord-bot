import discord
from discord.ext import commands

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
            embed = discord.Embed.from_dict(
                {
                    "title": str(len(data["stars"])) + " 🌟",
                    "author": {
                        "name": reaction_message.author.display_name,
                        "icon_url": reaction_message.author.display_avatar.url if reaction_message.author.display_avatar else discord.DefaultAvatar.blurple
                    },
                    "color": int(discord.Colour.gold()),
                    "description": reaction_message.content + f"\n\n[Jump to message]({reaction_message.jump_url})",
                    "timestamp": reaction_message.created_at.isoformat(),
                }
            )

            if reaction_message.attachments:
                embed.set_image(url=reaction_message.attachments[0].url)

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

        reaction_channel = await self.client.fetch_channel(payload.channel_id)
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

Cog = Starboard