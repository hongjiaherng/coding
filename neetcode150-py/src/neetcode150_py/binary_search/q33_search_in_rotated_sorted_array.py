from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        """
        T(n) = O(log n)
        S(n) = O(1)

        M >= R: [L ... M] is guaranteed to be the larger portion and in correct order
        M < R: [M ... R] is guaranteed to be the smaller portion and in correct order

        target = 8

        [4, 5, 6, 7, 8, 1, 2, 3]
         |        |           |
         L        M           R

        [4, 5, 6, 7, 8, 1, 2, 3]
                     |  |     |
                     L  M     R

        [4, 5, 6, 7, 8, 1, 2, 3]
                     |
                     L
                     R
                     M

        target = 5

        [4, 5, 6, 7, 0, 1, 2]
         |        |        |
         L        M        R


        [4, 5, 6, 7, 0, 1, 2]
         |  |  |
         L  M  R

        target = 3

        [4, 5, 6, 7, 0, 1, 2]
         |        |        |
         L        M        R

        [4, 5, 6, 7, 0, 1, 2]
                     |  |  |
                     L  M  R

        [4, 5, 6, 7, 0, 1, 2]
                     |
                     L
                     R
                     M

        [4, 5, 6, 7, 0, 1, 2]
                     |  |
                     R  L

        """

        left, right = 0, len(nums) - 1

        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] >= nums[right]:
                # M >= R: [L ... M] is guaranteed to be the larger portion and in correct order
                # Check if the target is within [L ... M]
                if nums[left] <= target < nums[mid]:
                    right = mid - 1
                else:
                    left = mid + 1
            else:  # nums[mid] < nums[right]
                # M < R: [M ... R] is guaranteed to be the smaller portion and in correct order
                # Check if the target is within [M ... R]
                if nums[mid] < target <= nums[right]:
                    left = mid + 1
                else:
                    right = mid - 1

        return -1
