from typing import List


class Solution:
    def encode(self, strs: List[str]) -> str:
        encoded = ""
        for s in strs:
            encoded += str(len(s)) + "#" + s
        return encoded

    def decode(self, s: str) -> List[str]:
        decoded: List[str] = []
        i = 0

        while i < len(s):
            j = i
            while s[j] != "#":
                j += 1
            wordlen = int(s[i:j])
            decoded.append(s[j + 1 : j + 1 + wordlen])
            i = j + wordlen + 1
        return decoded

    # def decode_(self, s: str) -> List[str]:
    #     decoded: List[str] = []

    #     if len(s) == 0:
    #         return decoded

    #     i = 0
    #     while i < len(s):
    #         wordlen_str = ""
    #         while i < len(s) and s[i].isdigit():
    #             wordlen_str += s[i]
    #             i += 1
    #         if s[i] == "#":
    #             i += 1

    #         wordlen = int(wordlen_str)
    #         decoded.append(s[i : i + wordlen])
    #         i += wordlen
    #     return decoded
