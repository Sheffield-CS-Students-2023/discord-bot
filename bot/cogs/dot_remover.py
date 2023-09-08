import discord
from discord.ext import commands
import re
from typing import Union

class DotRemover(commands.Cog):

    def __init__(self, client: commands.Bot):
        self.client = client

    def _find_if_dot(self, text: str) -> Union[str, None]:
        """Attempts to find any variations and tricks using markdown, unicode characters etc to discuise a dot at the end of the last sentence in text"""

        # Create list of possible unicode characters that look like dots (as characters, not \u+something eg "." not "\u002E")
        dots = [".", "·", "․", "‧", "⋅", "・", "⸱", "◌ׅ", "ᐧ", "⏺", "●", "⚬", "⦁", "⸰"]

        joined_dots =  "\\" + "".join(dots)
        if re.findall(rf"[{joined_dots}] ", text): # If the text has more than a single sentence the last dot does not need to be removed.
            return

        # Loop through all possible dots
        for dot in dots:
            # If the dot is found, return the text without dot
            if text.endswith(dot) and not text.endswith(dot + dot + dot):
                return text[:-1]

        # Create list of possible characters to surround dots that become invisible with markdown
        markdown = ["\*", "_", "~", "`"]

        # Loop through all possible markdown
        for dot in dots:
            # Loop through possible similar characters
            for mark in markdown:
                # If the dot is found, return the text without dot
                dot = dot if dot != '.' else '\.'
                if re.findall(rf"{mark}+{dot}{mark}+", text):
                    text = re.sub(rf"{mark}+{dot}{mark}+", "", text)
                    return text

        return None # Message is dot free!!

    @commands.Cog.listener()
    async def on_message(self, message: discord.Message):
        """If the last symbol of a message is a dot (unless 3 dots), remove dot, delete message and resend"""
        if message.author.bot: return

        # Figure out if it ends with a single dot and only has one sentence using rege
        if (text := self._find_if_dot(message.content)):
            # Create a webhook with the same name and avatar as the user, providing the avatar as bytes
            webhook = await message.channel.create_webhook(name=message.author.display_name, avatar=await message.author.avatar.read())
            # Send the message without the dot
            await webhook.send(text)
            # Delete original message
            await message.delete()
            # Delete the webhook
            await webhook.delete()

    @commands.Cog.listener()
    async def on_message_edit(self, before: discord.Message, after: discord.Message):
        """If the last symbol of a message is a dot (unless 3 dots), remove dot, delete message and resend"""
        if before.author.bot: return

        # Figure out if it ends with a single dot and only has one sentence using rege
        if (text := self._find_if_dot(after.content)):
            # Create a webhook with the same name and avatar as the user, providing the avatar as bytes
            webhook = await after.channel.create_webhook(name=after.author.display_name, avatar=await after.author.avatar.read())
            # Send the message without the dot
            await webhook.send(text)
            # Delete original message
            await after.delete()
            # Delete the webhook
            await webhook.delete()

Cog = DotRemover