import re
from discord.ext import commands
from typing import Union

from bot.bot import Bot
from bot.static.constants import API_TOKEN, EVAL_LANGS, ALIASES

class Eval(commands.Cog):

    def __init__(self, client: Bot):
        self.client = client
        self.url = "https://code-compiler10.p.rapidapi.com/"

    def get_lang(self, input: str) -> Union[str, None]:
        """Parse the language input into one of the allowed options"""
        for lang, aliases in ALIASES.items():
            if input.lower() in [lang, *aliases]:
                return lang
            
        return None

    @commands.command()
    async def eval(self, ctx: commands.Context, lang: str, *, code: str):
        """Evaluate some code"""

        lang = self.get_lang(lang)# Parse argument

        if not lang:
            return await ctx.send("Invalid language")
        
        # remove markdown from code using regex
        code = re.sub(r"```.*\n", "", code, 1)
        code = re.sub(r"```", "", code[::-1], 1)[::-1]

        payload = {
            "langEnum": EVAL_LANGS, # idk why I need to give this
            "lang": lang,
            "code": code,
            "input": ""
        }
        headers = {
            "content-type": "application/json",
            "x-compile": "rapidapi",
            "Content-Type": "application/json",
            "X-RapidAPI-Key": API_TOKEN,
            "X-RapidAPI-Host": "code-compiler10.p.rapidapi.com"
        }

        response = await self.client.session.post(self.url, json=payload, headers=headers)

        if response.status != 200:
            return await ctx.send("The following error occured: " + await response.text())
        
        data = await response.json()
        await ctx.send("```" + lang + "\n" + data["output"] + "\n```")

Cog = Eval