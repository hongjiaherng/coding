from math import inf
from typing import List, Dict


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        """
        T(n) = O(n)
        S(n) = O(2n) = O(n)

        Initial buckets:
        [[], [], [], [], [], []]

        Table:
        {
          1: 3,
          2: 2,
          3: 1,
        }

        Resulting buckets:
        [[3], [2], [1], [], [], []]
        """
        buckets: List[List[int]] = [[] for _ in range(len(nums))]
        table: Dict[int, int] = {}
        topk: List[int] = []

        for n in nums: # O(n)
            table[n] = table.get(n, 0) + 1

        for k_, v in table.items(): # O(n)
            buckets[v - 1].append(k_)

        for i in range(len(buckets) - 1, -1, -1): # O(n)
            for stuff in buckets[i]:
                topk.append(stuff)
                if len(topk) == k:
                    return topk

        return topk

    def topKFrequent1(self, nums: List[int], k: int) -> List[int]:
        """
        T(n) = O(n + kn)
        S(n) = O(n)
        """
        table: Dict[int, int] = {}
        topk: List[int] = []

        for n in nums:  # O(n)
            table[n] = table.get(n, 0) + 1

        for _ in range(k):  # O(kn)
            max_v = -inf
            max_k = -1
            for key in table:
                if max_v < table[key]:
                    max_v = table[key]
                    max_k = key

            # Reset val of current max
            topk.append(max_k)
            table.pop(max_k)

        return topk
