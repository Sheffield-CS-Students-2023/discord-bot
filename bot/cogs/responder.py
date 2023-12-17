#Written in loving memory of Henry Kissenger, a truer friend Cambodia never did have <3
from discord.ext import commands
import discord
from bot.static.constants import RESPONDENTS, GUILD_ID

class Responder(commands.cog):

    def __init__(self, client: commands.Bot):
        self.client = client

    @property
    def guild(self) -> discord.Guild:
        return self.client.get_guild(GUILD_ID)

    @commands.Cog.listener()
    async def on_message(self, message):
        """When a message gets sent to any server"""


        #If message is send by a marked respondant...
        if message.author in RESPONDENTS:
            if (message.clean_content) > 100:   #Jakeypoo has 100 chars to articulate himself before he is smitten
                await message.channel.send()
                message.channel.send(discord.utils.get(self.guild.stickers, name = "Shut up jake"))

                await message.author.timeout(until=datetime.timedelta(minutes = 5), reason="Unappreciated yapping") #timeout


Cog = Responder