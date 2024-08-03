# Grab Machine Learning Enginner - Technical Round


def remove_duplicates(s: str) -> str:
    prev_s = s
    while True:
        cur_s = ""
        i = 0
        while i < len(prev_s):
            if i < len(prev_s) - 1 and prev_s[i] == prev_s[i + 1]:
                while i < len(prev_s) - 1 and prev_s[i] == prev_s[i + 1]:
                    i += 1
            else:
                cur_s += prev_s[i]
            i += 1
        if cur_s == prev_s:
            break
        prev_s = cur_s
    return prev_s


if __name__ == "__main__":
    print(remove_duplicates("abbbacdaadce"))
