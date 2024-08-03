from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        """
        Reuse memory

        T(n) = O(2n) = O(n)
        S(n) = O(1)
        """
        ans = [1] * len(nums)

        prefix = 1
        for i in range(len(nums)):
            ans[i] = prefix
            prefix *= nums[i]

        suffix = 1
        for i in range(len(nums) - 1, -1, -1):
            ans[i] *= suffix
            suffix *= nums[i]

        return ans

    def productExceptSelf2(self, nums: List[int]) -> List[int]:
        """
        Modify input array to store intermediate result

        T(n) = O(3n) = O(n)
        S(n) = O(1)
        """
        ans = nums.copy()

        for i in range(1, len(nums)):
            nums[i] *= nums[i - 1]

        for i in range(len(nums) - 2, -1, -1):
            ans[i] *= ans[i + 1]

        for i in range(len(nums)):
            if i == 0:
                ans[i] = ans[i + 1]
            elif i == len(nums) - 1:
                ans[i] = nums[i - 1]
            else:
                ans[i] = ans[i + 1] * nums[i - 1]

        return ans

    def productExceptSelf1(self, nums: List[int]) -> List[int]:
        """
        T(n) = O(3n) = O(n)
        S(n) = O(2n) = O(n)
        """
        ans = [1] * len(nums)
        prefix = nums.copy()
        suffix = nums.copy()

        for i in range(1, len(nums)):
            prefix[i] *= prefix[i - 1]

        for i in range(len(nums) - 2, -1, -1):
            suffix[i] *= suffix[i + 1]

        for i in range(len(nums)):
            if i < len(nums) - 1:
                ans[i] *= suffix[i + 1]
            if i > 0:
                ans[i] *= prefix[i - 1]

        return ans
