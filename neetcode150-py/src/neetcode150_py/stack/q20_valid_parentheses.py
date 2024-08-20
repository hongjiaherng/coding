from typing import List


class Solution:
    def isValid(self, s: str) -> bool:
        """
        T(n) = O(n)
        S(n) = O(n)
        """
        open2close = {"(": ")", "[": "]", "{": "}"}
        stack: List[str] = []

        for p in s:
            if p in open2close:  # p is open
                stack.append(p)
            else:  # p is close
                if stack and stack[-1] in open2close and open2close[stack[-1]] == p:
                    stack.pop()
                else:
                    return False

        return not stack
