from discord.ext import commands
import discord
from bot.static.constants import ROLE_MESSAGE_ID, ROLES, COMPSOC_CHANNEL_ID, GUILD_ID
from bot.utils.functions import encrypt

class Roles(commands.Cog):

    def __init__(self, client: commands.Bot):
        self.client = client

    @property
    def guild(self) -> discord.Guild:
        return self.client.get_guild(GUILD_ID)
    
    @property
    def compsoc_channel(self) -> discord.TextChannel:
        return self.guild.get_channel(COMPSOC_CHANNEL_ID)

    @commands.Cog.listener()
    async def on_interaction(self, interaction: discord.Interaction):
        """Handles a buttonpress of the compsoc role button"""

        if interaction.message.id != ROLE_MESSAGE_ID and not interaction.data["custom_id"].startswith("compsoc"): # Ignore any interaction recieved without defined process
            return
        
        if interaction.data["custom_id"] == "compsoc": # If button press was auto role

            if ROLES["compsoc"] in [r.id for r in interaction.user.roles]:
                return await interaction.response.send_message("You already have the CompSoc role", ephemeral=True)
            
            embed = discord.Embed.from_dict({
                "title": interaction.user.display_name + " claims to be on the CompSoc committee",
                "description": "BUT ARE THEY REALLY? Press confirm to grant them the role, deny to deny.",
                "color": discord.Color.blurple().value,
                "thumbnail": {"url": interaction.user.avatar.url if interaction.user.avatar else interaction.user.default_avatar.url}
            })

            view = discord.ui.View()
            view.add_item(discord.ui.Button(label="Confirm", style=discord.ButtonStyle.green, custom_id=f"compsoc:{interaction.user.id}:confirm:,"))
            view.add_item(discord.ui.Button(label="Deny (0/2)", style=discord.ButtonStyle.red, custom_id=f"compsoc:{interaction.user.id}:deny:,"))

            await self.compsoc_channel.send(embed=embed, view=view)
            await interaction.response.send_message("Application sent. Please be patient and do not send another one.", ephemeral=True)

        elif interaction.data["custom_id"].startswith("compsoc"):
            _, user_id, action, denies = interaction.data["custom_id"].split(":") # Part the custom ID into information
            user = self.guild.get_member(int(user_id)) # Get (applying) user object

            if action == "confirm":
                if not user:
                    return await interaction.response.send_message("User not found", ephemeral=True)
                await user.add_roles(self.guild.get_role(ROLES["compsoc"])) # Add role to applicant
                await interaction.response.send_message("User added to role", ephemeral=True) # Confirm msg

                embed = interaction.message.embeds[0]
                embed.color = discord.Color.green().value
                embed.set_footer(text="User verified by " + interaction.user.display_name, icon_url=interaction.user.avatar.url if interaction.user.avatar else interaction.user.default_avatar.url) # Say who verified the user
                await interaction.message.edit(view=None, embed=embed) # Edit application message
                try: # Dm may fail due to closed dms
                    await user.send("You have been added to the CompSoc role. You can now see the CompSoc channels.")
                except discord.Forbidden:
                    pass

            elif action == "deny":
                if not user:
                    return await interaction.response.send_message("User not found", ephemeral=True)

                deniers = [d for d in denies.split(",") if d] # Get list of deniers

                if encrypt(interaction.user.id) in deniers:
                    return await interaction.response.send_message("You have already denied this user", ephemeral=True)
                
                if len(deniers) + 1 == 2: # If 2 deniers
                    await interaction.response.send_message("User denied", ephemeral=True)

                    embed = interaction.message.embeds[0]
                    embed.color = discord.Color.red().value
                    embed.title = user.display_name + " was denied the CompSoc role"
                    await interaction.message.edit(view=None, embed=embed) # Edit application message
                    try: # Dm may fail due to closed dms
                        await user.send("You have been denied the CompSoc role due to 2 committee members denying you.")
                    except discord.Forbidden:
                        pass

                else:

                    new_view = discord.ui.View()
                    for component in interaction.message.components[0].children:
                        d = component.to_dict()
                        del d["type"]
                        d["style"] = discord.ButtonStyle(d["style"])
                        if d["custom_id"] == interaction.data["custom_id"]:
                            d["label"] = f"Deny ({len(deniers) + 1}/2)"
                            d["custom_id"] += encrypt(interaction.user.id) + "," # Add denier to list
                        new_view.add_item(discord.ui.Button(**d))

                    await interaction.response.edit_message(view=new_view) # Edit application message

Cog = Roles