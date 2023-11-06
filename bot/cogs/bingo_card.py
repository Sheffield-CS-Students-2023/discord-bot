import discord
from discord.ext import commands
from io import BytesIO

from typing import List
from math import floor
from random import sample
from textwrap import fill
from PIL import Image, ImageDraw, ImageFont

from bot.static.constants import BINGO
from bot.bot import Bot

class Bingo(commands.Cog):

    def __init__(self, client: Bot):
        self.client = client

    def draw_square(self, text: str, font: str, size: int) -> Image.Image:
        '''
        Draw a single bingo square
        '''
        square = Image.new("RGB", (size, size), color=(66,69,73))
        draw = ImageDraw.Draw(square)

        draw.line([(0,0), (0, size)], fill=(255,255,255), width=2)
        draw.line([(0,size), (size, size)], fill=(255,255,255), width=2)
        draw.line([(size,size), (size, 0)], fill=(255,255,255), width=2)
        draw.line([(size,0), (0, 0)], fill=(255,255,255), width=2)

        # Fill is python's built in text wrapping feature. Isn't that amazing? Wow!
        # TODO?: Better alignment
        draw.text((4,4), fill(text, width=10), font=font, fill=(255,255,255))
        return square

    def draw_card(self, items_per_row: int, card_pixel_height: int, bingo_items: List[str]) -> BytesIO:
        '''
        Draw a bingo card by selecting random items from a list
        '''
        number_of_items: int = items_per_row**2
        square_size: int = floor(card_pixel_height / items_per_row)

        chosen_items: List[str] = sample(bingo_items, number_of_items)
        font = ImageFont.truetype("arial.ttf" if self.client.is_dev else "/usr/share/fonts/truetype/msttcorefonts/Arial.ttf", floor(card_pixel_height/20))
        card_image = Image.new("RGB", (card_pixel_height, card_pixel_height), color=(66,69,73))

        for i in range(number_of_items):
            if i == floor(number_of_items/2):
                # square_image = self.draw_square("Free Space", font, square_size) # The free space
                square_image = self.draw_square(chosen_items[i], font, square_size)
            else:
                square_image = self.draw_square(chosen_items[i], font, square_size)

            card_image.paste(square_image, ((i % items_per_row)*square_size, (i // items_per_row)*square_size))

        # Save image to io buffer
        buffer = BytesIO()
        card_image.save(buffer, format="PNG")
        buffer.seek(0)
        return buffer

    @commands.command()
    async def bingo(self, ctx: commands.Context):
        """Create a random bingo card"""
        buffer = self.draw_card(4, 300, BINGO)
        await ctx.send(file=discord.File(buffer, filename="bingo.png"))

Cog = Bingo
