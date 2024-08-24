from typing import List


class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        """
        T(n) = O(n)
        S(n) = O(n)

        If stack is empty or temperatures[i] <= temperatures[stack[-1]]
        - add current idx to stack
        Otherwise (temperatures[i] > temperatures[stack[-1]])
        - we found a warmer temperature day for stack[-1]-th day
        """
        days: List[int] = [0] * len(temperatures)
        stack: List[int] = []

        for i in range(len(temperatures)):
            while stack and temperatures[stack[-1]] < temperatures[i]:
                target_idx = stack.pop()
                days[target_idx] = i - target_idx
            stack.append(i)

        return days
