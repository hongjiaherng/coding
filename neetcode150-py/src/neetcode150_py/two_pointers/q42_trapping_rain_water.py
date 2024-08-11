from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        """
        T(n) = O(n)
        S(n) = O(1)
        """
        total_water = 0
        left, right = 0, len(height) - 1
        max_left, max_right = height[left], height[right]

        while left < right:
            if max_left <= max_right:
                left += 1
                max_left = max(max_left, height[left])
                total_water += max_left - height[left]
            else:
                right -= 1
                max_right = max(max_right, height[right])
                total_water += max_right - height[right]

        return total_water

    def trap2(self, height: List[int]) -> int:
        """
        T(n) = O(2n) = O(n)
        S(n) = O(2n) = O(n)
        """
        total_water = 0
        max_left = [0] * len(height)
        max_right = [0] * len(height)

        for i in range(len(height)):
            max_left[i] = 0 if i == 0 else max(max_left[i - 1], height[i - 1])
            max_right[len(height) - 1 - i] = (
                0
                if i == 0
                else max(max_right[len(height) - i], height[len(height) - i])
            )

        for i in range(len(height)):
            total_water += max(min(max_left[i], max_right[i]) - height[i], 0)

        return total_water
