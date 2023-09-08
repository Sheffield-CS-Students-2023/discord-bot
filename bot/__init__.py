import discord
from . import cogs
from bot.static.constants import TOKEN
import logging, sys
from aiohttp import ClientSession
from .bot import Bot

logger = logging.getLogger('discord')
logger.setLevel(logging.ERROR)
logging.getLogger('discord.http').setLevel(logging.ERROR)

logging.basicConfig(stream=sys.stdout, level=logging.INFO, format='[%(asctime)s:%(levelname)s:%(name)s] %(message)s', datefmt='%Y-%m-%d %H:%M:%S')

async def main():
    session = ClientSession()
    intents = discord.Intents.all()
    bot = Bot(session=session, command_prefix="!", intents=intents)

    await bot.load_extension("jishaku")

    # Setup cogs.
    for cog in cogs.all_cogs:
        await bot.add_cog(cog.Cog(bot))

    await bot.start(TOKEN)