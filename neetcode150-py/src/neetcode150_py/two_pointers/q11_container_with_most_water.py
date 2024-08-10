from typing import List


class Solution:
    """
    Seems like a constraint optimization problem.
    Given h, find i and j s.t. max_{i, j} {|i - j| * min(h_i, h_j)}
    """

    def maxArea(self, height: List[int]) -> int:
        """
        T(n) = O(n)
        S(n) = O(1)
        """
        max_area = 0
        left, right = 0, len(height) - 1
        while left < right:
            max_area = max(
                max_area, min(height[left], height[right]) * abs(left - right)
            )
            if height[left] < height[right]:
                left += 1
            else:
                right -= 1
        return max_area

    def brute_force_max_area(self, height: List[int]) -> int:
        """
        T(n) = O(n^2)
        S(n) = O(1)
        """
        max_a = 0
        for i in range(len(height) - 1):
            for j in range(i + 1, len(height)):
                w = j - i
                h = min(height[i], height[j])
                a = w * h
                if a > max_a:
                    max_a = a
        return max_a
