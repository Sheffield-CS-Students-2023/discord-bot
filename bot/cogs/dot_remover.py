import discord
from discord.ext import commands
import re
from typing import Union

class DotRemover(commands.Cog):

    def __init__(self, client: commands.Bot):
        self.client = client

    def _remove_whitespace_from_end(self, text: str, dot: str) -> str:
        """Removes all things discord markdown renders as whitespace from the end of the text"""
        whitespace = ["_ _"]
        for char in whitespace:
            # Use regex to remove all occurances of char after dot
            text = re.sub(rf"{dot}([ `]*{char})+", dot, text)
        return text

    def _find_if_dot(self, text: str) -> Union[str, None]:
        """Attempts to find any variations and tricks using markdown, unicode characters etc to discuise a dot at the end of the last sentence in text"""

        # Create list of possible unicode characters that look like dots (as characters, not \u+something eg "." not "\u002E")
        dots = [".", "·", "․", "‧", "⋅", "・", "⸱", "◌ׅ", "ᐧ", "⏺", "●", "⚬", "⦁", "⸰"]

        joined_dots =  "\\" + "".join(dots)
        if re.findall(rf"[{joined_dots}] ", text): # If the text has more than a single sentence the last dot does not need to be removed.
            return

        # Loop through all possible dots
        for dot in dots:
            new_text = self._remove_whitespace_from_end(text, dot)
            # If the dot is found, return the text without dot
            if new_text.endswith(dot) and not new_text.endswith(dot + dot + dot):
                # Check if second to last character is a dot
                if new_text[-2] == dot:
                    # If it is, remove both dots
                    return new_text[:-2]
                return new_text[:-1]
            elif re.findall(rf"{dot} *`$") and new_text.count("`") == 2 and not re.findall(rf"{dot}{dot}{dot} *`$"):
                # Check if third to last character is a dot
                if new_text[-3] == dot:
                    # If it is, remove both dots
                    return re.sub(rf"{dot}{dot} *`$", "`", new_text)
                return re.sub(rf"{dot} *`$", "`", new_text)

        # Create list of possible characters to surround dots that become invisible with markdown
        markdown = ["\*", "_", "~", "`"]

        # Loop through all possible markdown
        for dot in dots:
            # Loop through possible similar characters
            for mark in markdown:
                # If the dot is found, return the text without dot
                new_text = self._remove_whitespace_from_end(text, dot)
                dot = dot if dot != '.' else '\.'
                one_to_two = "{1,2}"
                if re.findall(rf"{mark}+{dot}{one_to_two}{mark}+", new_text):
                    new_text = re.sub(rf"{mark}+{dot}{one_to_two}{mark}+", "", new_text)
                    return new_text

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