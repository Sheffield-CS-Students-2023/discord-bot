import discord
from discord.ext import commands
from io import BytesIO
from PIL import Image, ImageDraw, ImageFont
from typing import List
from random import shuffle

from bot.static.constants import BINGO
from bot.bot import Bot

class Bingo(commands.Cog):

    def __init__(self, client: Bot):
        self.client = client

    def create(self, values=List[str], x=5, y=5) -> BytesIO:
        """Draw a XxY bingo sheet with the given values using pillow and return it as a bytesIO object"""

        values: List[str] = values
        # Only take a random sample of x*y-1 values
        shuffle(values)
        values = values[:x * y - 1]

        # Create a new image
        image = Image.new("RGB", (x * 100, y * 100), color="white")
        draw = ImageDraw.Draw(image)

        # Draw the grid
        for i in range(0, 500, 100):
            draw.line((0, i, 500, i), fill="black", width=5)
            draw.line((i, 0, i, 500), fill="black", width=5)

        # Load the font
        font = ImageFont.truetype("arial.tff" if self.client.is_dev else "/usr/share/fonts/truetype/msttcorefonts/Arial.ttf", 15)

        # Add an "Free Spot" string to the middle of the list
        values.insert(len(values) // 2, "Free spot")
        # For some reason this is just a blank space and idk why but it works so I'm not gonna question it TODO: Figure out why this workn't

        # Add text to the image
        for i, value in enumerate(values):
            x = (i % x) * 100 + 50
            y = (i // y) * 100 + 50
            # if text is longer than box width, split it into multiple lines (as a loop) but does not split words
            if len(value) > 10:
                words = value.split(" ")
                lines = []
                line = ""
                for word in words:
                    if len(line + word) > 10:
                        lines.append(line)
                        line = ""
                    line += word + " "
                lines.append(line)
                for j, line in enumerate(lines):
                    draw.text((x, y + (j - len(lines) // 2) * 15), line, fill="black", font=font, anchor="mm")

        # Save image to io buffer
        buffer = BytesIO()
        image.save(buffer, format="PNG")
        buffer.seek(0)
        return buffer

    @commands.command()
    async def bingo(self, ctx: commands.Context):
        """Create a random bingo card"""
        buffer = self.create(BINGO, 3, 3)
        await ctx.send(file=discord.File(buffer, filename="bingo.png"))

Cog = Bingo