from PIL import Image, ImageDraw, ImageFont, _imaging
from typing import List
from io import BytesIO
from random import shuffle
import os

class BingoCard:

    def __init__(self, values=List[str], x=5, y=5):
        self.values: List[str] = values
        # Only take a random sample of x*y-1 values
        shuffle(self.values)
        self.values = self.values[:x * y - 1]

        self.x: int = x
        self.y: int = y

    def create(self) -> BytesIO:
        """Draw a XxY bingo sheet with the given values using pillow and return it as a bytesIO object"""

        # Create a new image
        image = Image.new("RGB", (self.x * 100, self.y * 100), color="white")
        draw = ImageDraw.Draw(image)

        # Draw the grid
        for i in range(0, 500, 100):
            draw.line((0, i, 500, i), fill="black", width=5)
            draw.line((i, 0, i, 500), fill="black", width=5)

        # Get current file path
        path = os.path.dirname(os.path.realpath(__file__))

        # remove last two folders name from path
        path = os.path.dirname(path)
        path = os.path.dirname(path)

        # Thus save arial.tff path to variable
        font_path = os.path.join(path, "arial.ttf")

        print(font_path)

        # Load the font
        font = ImageFont.truetype(font_path, 15)

        # Add an "Free Spot" string to the middle of the list
        self.values.insert(len(self.values) // 2, "Free spot")
        # For some reason this is just a blank space and idk why but it works so I'm not gonna question it TODO: Figure out why this workn't

        # Add text to the image
        for i, value in enumerate(self.values):
            x = (i % self.x) * 100 + 50
            y = (i // self.y) * 100 + 50
            # if text is longer than box width, split it into multiple lines (as a loop) but does not split words
            if len(value) > 10:
                words = value.split(" ")
                lines = []
                line = ""
                for word in words:
                    if len(line + word) > 10:
                        lines.append(line)
                        line = ""
                    line += word + " "
                lines.append(line)
                for j, line in enumerate(lines):
                    draw.text((x, y + (j - len(lines) // 2) * 15), line, fill="black", font=font, anchor="mm")

        # Save image to io buffer
        buffer = BytesIO()
        image.save(buffer, format="PNG")
        buffer.seek(0)
        return buffer