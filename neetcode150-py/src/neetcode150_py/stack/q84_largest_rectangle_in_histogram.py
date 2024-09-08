from typing import List, Tuple


class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        """
        T(n) = O(n)
        S(n) = O(n)
        """
        area = 0
        stack: List[Tuple[int, int]] = []  # pair: (index, height)

        for i, h in enumerate(heights):
            start = i
            while stack and stack[-1][1] > h:
                prev_i, prev_h = stack.pop()  # start idx and height of the bin
                area = max(area, prev_h * (i - prev_i))
                start = prev_i
            stack.append((start, h))

        # Eventually, the stack is monotonically increasing => all bins can be extended to the end
        while stack:
            prev_i, prev_h = stack.pop()
            area = max(area, prev_h * (len(heights) - prev_i))

        return area

    def largestRectangleAreaQuadratic(self, heights: List[int]) -> int:
        """
        T(n) = O(n^2)
        S(n) = O(1)
        """
        area = 0
        for i in range(len(heights)):  # O(n)
            height = heights[i]
            for j in range(i, len(heights)):  # O(n)
                width = j - i + 1
                height = min(height, heights[j])
                area = max(area, width * height)
        return area
