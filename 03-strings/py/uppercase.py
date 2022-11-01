def capitalize(word):
    if word:
        first = word[0]
        if first.isascii():
            return first.upper() + word[1:]
        return word
    return ""


people = ["homer", "marge", "bart", "9lisa", "", "maggie"]
for name in people:
    print(name, '->', name.capitalize())
    print(name, '->', capitalize(name))
    print("")