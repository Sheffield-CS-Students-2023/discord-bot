import discord
from discord.ext import commands
from urllib.parse import quote


class LaTeX(commands.Cog):
    
        def __init__(self, client: commands.Bot):
            self.client = client
    
        @discord.app_commands.command()
        async def latex(self, interaction: discord.Interaction, *, latex: str):
            """Render LaTeX"""

            # Use the rest API https://latex.codecogs.com/png.latex?
            latex = quote(latex).replace(" ", "&space;")
            await interaction.response.send_message("https://latex.codecogs.com/png.latex?\\dpi{250}\\bg{white}" + latex)

Cog = LaTeX
