from typing import List


class MinStack:
    """
    T(n) = O(1) for all operations
    """
    def __init__(self) -> None:
        self.stack: List[int] = []
        self.min_stack: List[int] = []

    def push(self, val: int) -> None:
        if not self.min_stack or val < self.min_stack[-1]:
            self.min_stack.append(val)
        else:
            self.min_stack.append(self.min_stack[-1])
        self.stack.append(val)

    def pop(self) -> None:
        if not self.stack:
            raise Exception()
        self.min_stack.pop()
        self.stack.pop()

    def top(self) -> int:
        if not self.stack:
            raise Exception()
        return self.stack[-1]

    def getMin(self) -> int:
        if not self.stack:
            raise Exception()
        return self.min_stack[-1]


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(val)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()
