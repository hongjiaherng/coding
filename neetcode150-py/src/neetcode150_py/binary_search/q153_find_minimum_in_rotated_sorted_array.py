from typing import List


class Solution:
    def findMin(self, nums: List[int]) -> int:
        """
        T(n) = O(log n)
        S(n) = O(1)

        [3, 4, 5, 1, 2]
         |     |     |
         L     M     R

        M >= R: Right portion is smaller, search right (left = mid + 1), mid is surely not the smallest

        [3, 4, 5, 1, 2]
                  |  |
                  L  R
                  M

        M < R: Left portion is smaller, search left (right = mid), mid itself might be the smallest.

        [3, 4, 5, 1, 2]
                  |
                  L
                  R
                  M

        M >= R: Right portion is smaller, search right (left = mid + 1), left > right, break while loop

        Answer: M

        ===================================================

        [11, 13, 15, 17]
         |   |       |
         L   M       R

        M < R: Left portion is smaller, search left (right = mid), mid itself might be the smallest.

        [11, 13, 15, 17]
         |   |
         L   R
         M

        M < R: Left portion is smaller, search left (right = mid), mid itself might be the smallest.

        [11, 13, 15, 17]
         |
         L
         R
         M

        M >= R: Right portion is smaller, search right (left = mid + 1), left > right, break out the while loop.

        Answer: M

        """

        left, right = 0, len(nums) - 1

        while left <= right:
            mid = left + (right - left) // 2

            if nums[mid] >= nums[right]:
                left = mid + 1
            else:
                right = mid

        return nums[mid]
