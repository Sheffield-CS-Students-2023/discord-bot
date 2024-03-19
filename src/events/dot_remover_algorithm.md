# How does dot remover work?
For the sake of any future contributors, but also for my own sanity, I will explain how the dot remover algorithm works.

## Considerations
The dot remover algorithm has a single, simple goal: if a string that is passed to it that looks like (this looks like is the challenge of the algorithm) it ends with a dot, but does not contain more than one sentence, remove the dot.

### Why? Why do this? Why?
The main idea behind this was a joke. The joke is simply that sending a single sentence text with a dot at the end, while gramatically correct, is kind of unusal and "weird" nowadays (to me it always looks passive agressive when someone does this). But as people started finding workarounds to a simple `if (text.endsWith("."))` check, I started stepping up my game. And oh boy are there a lot of workarounds. But it has become a challenge for me to keep up with them, training my knowledge about regex and algorithms.

## The algorithm
The algorithm goes through a few steps, each designed to catch a specific type of workaround. 

![Image](https://i.imgur.com/ckXUO2E.png)

This passes all test cases I have come up with. If you find a case that is not covered, please open an issue or a pull request.