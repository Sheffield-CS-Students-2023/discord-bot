from typing import List, Optional, Dict, Union

from bot.static.constants import STARBORD

class Starboard:

    cache = {}

    @classmethod 
    def __get_cache(cls):
        """Returns a cached object"""
        return cls.cache

    def __new__(cls, *args, **kwargs):
        existing = cls.__get_cache()
        if existing:
            return existing
        return super().__new__(cls)

    def __init__(self):
        if self.cache:
            return 

        # Establish connection to the database
        self.data = STARBORD.find({})
        self.data: List[Dict[str, Union[List[int], None, int]]] = [i for i in self.data]

        # Cache the data
        self.cache = self.data

    def __getitem__(self, key: int):
        """Get a starboard message by its id"""
        for i in self.data:
            if i["_id"] == key:
                return i
        raise KeyError(f"Starboard message with id {key} not found")
    
    def __setitem__(self, key: int, value: dict):
        """Set a starboard message by its id"""
        for i in self.data:
            if i["_id"] == key:
                i = value
                STARBORD.update_one({"_id": key}, {"$set": value})
                return
        raise KeyError(f"Starboard message with id {key} not found")
    
    def __iter__(self):
        """Iterate through all starboard messages"""
        return iter(self.data)
    
    def __len__(self):
        """Return the amount of starboard messages"""
        return len(self.data)
    
    def __contains__(self, key: int):
        """Check if a starboard message with the given id exists"""
        for i in self.data:
            if i["id"] == key:
                return True
        return False

    def __repr__(self):
        return f"<Starboard: {len(self.data)} messages>"
    
    def __str__(self):
        return f"Starboard with {len(self.data)} messages"
    
    def append(self, value: dict):
        """Add a starboard message to the database"""
        self.data.append(value)
        STARBORD.insert_one(value)

    def add_star(self, key: int, user_id: int, author_id: int) -> dict:
        """Add a star to a starboard message"""
        found = False

        for i in self.data:
            if i["_id"] == key:
                i["stars"].append(user_id)
                STARBORD.update_one({"_id": key}, {"$set": {"stars": i["stars"]}})
                found = True
                break
                
        if not found:
            self.append({"_id": key, "stars": [user_id], "starboard_id": None, "author_id": author_id})

        return self[key]
        

    def remove_star(self, key: int, user_id: int) -> Optional[dict]:
        """Remove a star from a starboard message. If no starboard message is found, return None"""
        for i in self.data:
            if i["_id"] == key:
                i["stars"].remove(user_id)
                STARBORD.update_one({"_id": key}, {"$set": {"stars": i["stars"]}})
                return i