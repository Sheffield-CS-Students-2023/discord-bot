import json

with open("config.json", "r") as f:
    config = json.load(f)

TOKEN = config["token"]
API_TOKEN = config["api_token"]

GUILD_ID = 1093287471162335244

ROLE_MESSAGE_ID = 1149121682104139836
COMPSOC_CHANNEL_ID = 1149116888517984326

ALIASES = {
    "typescript": ["ts"],
    "c_cpp": ["c++", "cpp"],
    "csharp": ["c#"],
    "nodejs": ["js", "javascript"],
    "python": ["py"],
    "goland": ["go"],
    "bash": ["sh"],
    "c": [],
    "java": [],
    "kotlin": [],
    "php": [],
    "r": [],
    "ruby": [],
    "perl": [],
    "swift": [],
    "fortran": []
}

EVAL_LANGS = ["php", "python", "c", "c_cpp", "csharp", "kotlin", "golang", "r", "java", "typescript", "nodejs", "ruby", "perl", "swift", "fortran", "bash"]

WELCOME_TEXT = "" + \
"""Welcome to the 2023 Sheffield Computer Science Discord server **{}**!

This is a place to hang out, chat, organise meetings, and get help with your course.
If you are not from this year, make sure to select the appropriate role in the server's role menu.

Enjoy!
"""

ROLES = {
    "compsoc": 1149059391753048174
}