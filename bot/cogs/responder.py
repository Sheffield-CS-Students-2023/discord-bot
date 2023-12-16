#Written in loving memory of Henry Kissenger, a truer friend Cambodia never did have <3

import discord
from bot.static.constants import RESPONDENTS

class Responder(commands.cog):

    def __init__(self, client: commands.Bot):
        self.client = client

    @commands.Cog.listener()
    async def on_message(message):
        """When a message gets sent to any server"""


        #If message is send by a marked respondant...
        if message.author in RESPONDENTS:
            message.channel.send('https://i.imgur.com/HTd7xNP.png') #Not using actual sticker since apparently it's not in the API, shit company


Cog = Responder