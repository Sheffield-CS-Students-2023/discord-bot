import json
from pymongo import MongoClient

with open("config.json", "r") as f:
    config = json.load(f)

CLUSTER = MongoClient(config["mongodb"])
DB = CLUSTER["discord"]
STARBORD = DB["starboard"]

TOKEN = config["token"]
API_TOKEN = config["api_token"]

MIN_STARS = 3
STARBOARD_CHANNEL_ID = 1162423699455090748

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

BINGO = [
    "repeats a slide",
    "mentions testing",
    "shows some weird diagram",
    "mentions planes",
    "goes on a 5 min tangent",
    "mentions feedback",
    "mentions waterfall",
    "someone goes to sleep in the lecture",
    "mentions \"client\" 5 times",
    "someone plays a game on their laptop",
    "going back to this bullshit analogy",
    "people leaving during break",
    "discord is active af"
]