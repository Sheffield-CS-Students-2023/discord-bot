from discord.ext import commands
import discord

from bot.static.constants import WELCOME_TEXT

class Welcome(commands.Cog):

    def __init__(self, client: commands.Bot):
        self.client = client

    @commands.Cog.listener()
    async def on_member_join(self, member: discord.Member):
        """Say hello to a new member upon joining"""

        try: # Dm may fail due to closed dms
            await member.send(WELCOME_TEXT.format(member.display_name))
        except discord.HTTPException:
            pass

Cog = Welcome