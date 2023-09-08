from aiohttp import ClientSession
from discord.ext import commands

class Bot(commands.Bot):

    def __init__(self, session: ClientSession, *args, **kwargs):
        self.session = session
        super().__init__(*args, **kwargs)

    async def close(self) -> None:
        await self.session.close()
        await super().close()

    async def setup_hook(self) -> None:
        print("Ready")
        await self.tree.sync()