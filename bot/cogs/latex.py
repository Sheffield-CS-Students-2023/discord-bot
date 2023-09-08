import discord
from discord.ext import commands
import matplotlib

matplotlib.rcParams.update(matplotlib.rcParamsDefault)
# matplotlib.rcParams["mathtext.fontset"] = True

from matplotlib import pyplot as plt
from io import BytesIO


class LaTeX(commands.Cog):
    
        def __init__(self, client: commands.Bot):
            self.client = client
    
        @discord.app_commands.command()
        async def latex(self, interaction: discord.Interaction, *, latex: str):
            """Render LaTeX"""

            await interaction.response.defer(ephemeral=False, thinking=True) # Response may take > 3 seconds
    
            plt.text(0.5, 0.5, latex, size=20, ha="center", va="center")
            plt.axis("off")

            fig = plt.gcf()

            # Save figure to buffer
            buf = BytesIO()
            fig.savefig(buf, format='png', bbox_inches='tight')

            # Clear figure
            plt.clf()
    
            # Seek to the beginning of the buffer
            buf.seek(0)

            await interaction.followup.send("Here's your LaTeX:", file=discord.File(buf, "latex.png"), ephemeral=False)

Cog = LaTeX
