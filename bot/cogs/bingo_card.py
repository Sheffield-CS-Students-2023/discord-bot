import discord
from discord.ext import commands

from bot.utils.classes import BingoCard
from bot.static.constants import BINGO

class Bingo(commands.Cog):

    def __init__(self, client):
        self.client = client

    @commands.command()
    async def bingo(self, ctx: commands.Context):
        """Create a random bingo card"""
        card = BingoCard(BINGO, 3, 3)
        buffer = card.create()
        await ctx.send(file=discord.File(buffer, filename="bingo.png"))

Cog = Bingo