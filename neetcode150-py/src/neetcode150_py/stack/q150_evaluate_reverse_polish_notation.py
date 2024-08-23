from typing import List


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        """
        T(n) = O(n)
        S(n) = O(n)
        """
        stack: List[int] = []
        for token in tokens:
            match token:
                case "+":
                    a, b = int(stack.pop()), int(stack.pop())
                    stack.append(b + a)
                case "-":
                    a, b = int(stack.pop()), int(stack.pop())
                    stack.append(b - a)
                case "*":
                    a, b = int(stack.pop()), int(stack.pop())
                    stack.append(b * a)
                case "/":
                    a, b = int(stack.pop()), int(stack.pop())
                    stack.append(int(b / a))
                case _:
                    stack.append(int(token))
        return stack[0]
