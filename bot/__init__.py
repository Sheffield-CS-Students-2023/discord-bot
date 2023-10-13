import discord
from . import cogs
from bot.static.constants import TOKEN
import logging, sys
from aiohttp import ClientSession
from .bot import Bot
import argparse

logger = logging.getLogger('discord')
logger.setLevel(logging.ERROR)
logging.getLogger('discord.http').setLevel(logging.ERROR)

logging.basicConfig(stream=sys.stdout, level=logging.INFO, format='[%(asctime)s:%(levelname)s:%(name)s] %(message)s', datefmt='%Y-%m-%d %H:%M:%S')

async def main():

    parser = argparse.ArgumentParser(description="CLI arguments for the bot")
    parser.add_argument("-d", "--development", help="Run the bot in development mode", action="store_const", const=True)

    parsed = parser.parse_args()

    session = ClientSession()
    intents = discord.Intents.all()
    bot = Bot(session=session, command_prefix="!", intents=intents)

    # Checks if the bot is a dev bot
    bot.is_dev = parsed.development

    await bot.load_extension("jishaku")

    # Setup cogs.
    for cog in cogs.all_cogs:
        await bot.add_cog(cog.Cog(bot))

    await bot.start(TOKEN)