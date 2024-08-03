from typing import Dict


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        """
        T(n) = O(2n) = O(n)
        S(n) = O(2n) = O(n)
        """
        if len(s) != len(t):
            return False

        count: Dict[str, int] = {}
        for char_s, char_t in zip(s, t):
            count[char_s] = count.get(char_s, 0) + 1
            count[char_t] = count.get(char_t, 0) - 1

        for c in count:
            if count[c] != 0:
                return False

        return True
