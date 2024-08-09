from typing import List


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        results: List[List[int]] = []
        nums.sort()
        for i in range(len(nums)):
            if i > 0 and nums[i] == nums[i - 1]:
                continue

            left, right = i + 1, len(nums) - 1
            while left < right:
                three_sum = nums[i] + nums[left] + nums[right]
                if three_sum > 0:
                    right -= 1
                elif three_sum < 0:
                    left += 1
                else:
                    results.append([nums[i], nums[left], nums[right]])
                    right -= 1
                    while left < right and nums[right] == nums[right + 1]:
                        right -= 1
        return results
