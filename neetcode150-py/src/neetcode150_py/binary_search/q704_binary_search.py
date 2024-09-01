from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        """
        T(n) = O(log n)
        S(n) = O(1)
        """
        left = 0
        right = len(nums) - 1

        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        return -1
