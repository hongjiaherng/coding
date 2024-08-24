from typing import List


class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        """
        T(n) = O(2^n); Define how many times the function calls itself. 2 times then O(2^n)
        S(n) = O(n)
        """
        res: List[str] = []

        def backtrack(n_open: int, n_close: int, stack: List[str]) -> None:
            if n_open == n_close == n:
                res.append("".join(stack))
                return

            if n_open < n:
                stack.append("(")
                backtrack(n_open + 1, n_close, stack)
                stack.pop()

            if n_close < n_open:
                stack.append(")")
                backtrack(n_open, n_close + 1, stack)
                stack.pop()

        backtrack(0, 0, [])
        return res
