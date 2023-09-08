import discord
from . import cogs
from discord.ext import commands
from bot.static.constants import TOKEN
import logging, sys

logger = logging.getLogger('discord')
logger.setLevel(logging.ERROR)
logging.getLogger('discord.http').setLevel(logging.ERROR)

logging.basicConfig(stream=sys.stdout, level=logging.INFO, format='[%(asctime)s:%(levelname)s:%(name)s] %(message)s', datefmt='%Y-%m-%d %H:%M:%S')

class Bot(commands.Bot):

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    async def setup_hook(self) -> None:
        print("Ready")
        await self.tree.sync()

async def main():
    intents = discord.Intents.all()
    bot = Bot(command_prefix="!", intents=intents)

    await bot.load_extension("jishaku")

    # Setup cogs.
    for cog in cogs.all_cogs:
        await bot.add_cog(cog.Cog(bot))

    await bot.start(TOKEN)