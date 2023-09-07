def encrypt(n: int, b: int = 10000) -> str:
    """Changes an integer into base 10000 but with my own characters resembling numbers. This is used to turn a discord id into the smalles possible character length"""
    chars = "".join([chr(i) for i in range(b+1)][::-1])
    chars = chars.replace(":", "").replace(",", "") # These characters are indicators used in the ids so they should be not be available as characters

    if n == 0:
        return [0]
    digits = []
    while n:
        digits.append(int(n % b))
        n //= b
    return "".join([chars[d] for d in digits[::-1]])
