from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        """
        T(n) = O(log min(m, n))
        S(n) = O(1)

        nums1 = [1, 2, 3, 4]
        nums2 = [1, 2, 3, 4, 5, 6, 7, 9]
        half_len = 6

        ============
        Iteration 1
        ============

        [1, 2, 3, 4]
         |  |     |
         L  M1    R

        [1, 2, 3, 4, 5, 6, 7, 9]
                  |
                  M2

        M2 = half_len - (M1 + 1) - 1 = 6 - (1 + 1) - 1 = 3

        left_partition1       right_partition1
        [1, 2]                [3, 4]
            |                  |
            max_left1          min_right1

        left_partition2       right_partition2
        [1, 2, 3, 4]          [5, 6, 7, 8]
                  |            |
                  max_left2    min_right2

        max_left1 <= min_right2 && max_left2 <= min_right1 ?
        - NO, bad partition
        [{left_partition1, left_partition2}, {right_partition1, right_partition2}]
        [{ [1, 2] | [1, 2, 3, 4] }, { [3, 4] | [5, 6, 7, 8] }] # The max of left partition must be smaller than the min of right partition

        max_left1 > min_right2 ? NO

        max_left2 > min_right1 ? YES
        - reduce left partition 2 size, increase left partition 1 size
        - L = M1 + 1

        ============
        Iteration 2
        ============

        [1, 2, 3, 4]
               |  |
               L  R
               M1

        [1, 2, 3, 4, 5, 6, 7, 9]
               |
               M2

        M2 = half_len - (M1 + 1) - 1 = 6 - (2 + 1) - 1 = 2

        left_partition1 right_partition1
        [1, 2, 3]              [4]
               |                |
               max_left1        min_right1

        [1, 2, 3]              [4, 5, 6, 7, 9]
               |                |
               max_left2        min_right2

        max_left1 <= min_right2 && max_left2 <= min_right1 ? YES
        - Good partition
        [{ [1, 2, 3], [1, 2, 3] }, { [4], [4, 5, 6, 7, 9] }]

        median = 0.5 * (max(max_left1, max_left2) + min(min_right1, min_right2))
               = 0.5 * (max(3, 3) + min(4, 4))
               = 3.5

        """
        # Always make nums1 shorter than nums2
        if len(nums1) > len(nums2):
            nums1, nums2 = nums2, nums1

        total_len = len(nums1) + len(nums2)
        half_len = total_len // 2

        left, right = 0, len(nums1) - 1
        while True:
            partition1 = left + (right - left) // 2
            partition2 = half_len - (partition1 + 1) - 1

            max_left1 = nums1[partition1] if partition1 >= 0 else float("-inf")
            min_right1 = (
                nums1[partition1 + 1] if partition1 + 1 < len(nums1) else float("inf")
            )

            max_left2 = nums2[partition2] if partition2 >= 0 else float("-inf")
            min_right2 = (
                nums2[partition2 + 1] if partition2 + 1 < len(nums2) else float("inf")
            )

            if max_left1 <= min_right2 and max_left2 <= min_right1:
                if total_len % 2 == 1:  # Odd case
                    return min(min_right1, min_right2)
                return (max(max_left1, max_left2) + min(min_right1, min_right2)) / 2

            elif max_left1 > min_right2:
                right = partition1 - 1
            else:
                left = partition1 + 1
