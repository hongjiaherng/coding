from typing import List
import math


class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        """
        T(n) = O(n) + O(log(max(piles)) * n) = O(log(max(piles)) * n)
        S(n) = O(1)

        Search k from [1..11], determine the min k to satisfy h = 8
        k = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
             |     |     |  |
             l     m     r  h = 6 < 8
        """
        left, right = 1, max(piles)  # O(n)
        k = right

        while left <= right:
            k_potential = left + (right - left) // 2
            h_taken = 0  # hours taken to finish all bananas at the rate of k_potential
            for p in piles:
                h_taken += math.ceil(p / k_potential)
            if h_taken <= h:
                k = min(k, k_potential)
                right = k_potential - 1
            else:  # cur_h > h
                left = k_potential + 1

        return k
