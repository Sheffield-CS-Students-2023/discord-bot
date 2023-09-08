import discord
from discord.ext import commands
import re

class DotRemover(commands.Cog):

    def __init__(self, client: commands.Bot):
        self.client = client

    @commands.Cog.listener()
    async def on_message(self, message: discord.Message):
        """If the last symbol of a message is a dot (unless 3 dots), remove dot, delete message and resend"""

        # Figure out if it ends with a single dot and only has one sentence using rege
        if message.content.endswith(".") and not message.content.endswith("...") and not re.findall(r"\. ", message.content):
            # Create a webhook with the same name and avatar as the user, providing the avatar as bytes
            webhook = await message.channel.create_webhook(name=message.author.display_name, avatar=await message.author.avatar.read())
            # Send the message without the dot
            await webhook.send(message.content[:-1])
            # Delete original message
            await message.delete()
            # Delete the webhook
            await webhook.delete()

Cog = DotRemover