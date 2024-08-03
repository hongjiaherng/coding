from typing import List, Dict, Optional


class Solution:
    def twoSum(self, nums: List[int], target: int) -> Optional[List[int]]:
        """
        T(n) = O(n)
        S(n) = O(n)
        """
        val2idx: Dict[int, int] = {}
        for i in range(len(nums)):
            want = target - nums[i]
            if want in val2idx:
                return [i, val2idx[want]]
            val2idx[nums[i]] = i
        return None

    # def twoSum(self, nums: List[int], target: int) -> List[int]:
    #     """
    #     T(n) = O(n^2)
    #     S(n) = O(1)
    #     """
    #     for i in range(len(nums)):
    #         for j in range(i + 1, len(nums)):
    #             if nums[i] + nums[j] != target:
    #                 continue
    #             return [i, j]
    #     return []
