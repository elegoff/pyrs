if __name__ == "__main__":
    homer = "I want a donut"
    marge = homer.replace("donut", "kiss")

    print(f"Homer says : {homer}")
    print(f"Marge replies : {marge} instead")
    print(f"Marge adds : {marge} please")

    phrase_homer = "trying is the first step towards failure"
    for word in reversed(phrase_homer.split()):
        print(">", word)

    chars = [c for c in phrase_homer]
    chars = sorted(set(chars))

    s = ""
    for c in chars:
        s += c
        s += ", "

    trim = ' ,'
    trimmed_str = s.strip(trim)
    print("Used    chars:", s)
    print("Trimmed chars:", trimmed_str)