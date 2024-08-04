from typing import List, Dict


class Solution:
    """
    Constraint: T(n) = O(n)
    """

    def longestConsecutive(self, nums: List[int]) -> int:
        """
        T(n) = O(n)
        S(n) = O(n)
        """
        longest = 0
        nums_set = set(nums)

        for n in nums:
            if n - 1 in nums_set:  # It's not the leftmost number in the sequence
                continue

            cur_n = n
            while cur_n in nums_set:
                cur_n += 1
            longest = max(cur_n - n, longest)

        return longest

    def longestConsecutive1(self, nums: List[int]) -> int:
        """
        T(n) = O(n^2)
        S(n) = O(2n) = O(n)
        """
        if len(nums) == 0:
            return 0

        table: Dict[int, List] = {}
        nums_set = set(nums)
        for n in nums:
            cur_n = n
            while cur_n - 1 in nums_set:
                cur_n -= 1
            if cur_n not in table or (
                cur_n in table and len(table[cur_n]) < n + 1 - cur_n
            ):
                table[cur_n] = list(range(cur_n, n + 1))

        return max(len(x) for x in table.values())
