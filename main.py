import sys

sys.stdout.reconfigure(encoding="utf-8")


L2N: dict[str, int] = {
    "ا": 1,
    "آ": 1,
    "ب": 2,
    "ج": 3,
    "د": 4,
    "ه": 5,
    "و": 6,
    "ز": 7,
    "ح": 8,
    "ط": 9,
    "ی": 10,
    "ک": 20,
    "ل": 30,
    "م": 40,
    "ن": 50,
    "س": 60,
    "ع": 70,
    "ف": 80,
    "ص": 90,
    "ق": 100,
    "ر": 200,
    "ش": 300,
    "ت": 400,
    "ث": 500,
    "خ": 600,
    "ذ": 700,
    "ض": 800,
    "ظ": 900,
    "غ": 1000,
}


def read_file() -> list:
    output = ""
    with open("content.txt", "r", encoding="utf-8") as file:
        content = file.read()
        for word in file:
            output += word
    return content.split(" ")


def encode(word: str, small: bool = False):
    counter = 0
    for w in word:
        for l, n in L2N.items():
            if small and w == "س":
                counter += 0
            elif w == l:
                counter += n
    return counter


file = read_file()
print(file)

codin = [encode(i) for i in file]
print(codin)
