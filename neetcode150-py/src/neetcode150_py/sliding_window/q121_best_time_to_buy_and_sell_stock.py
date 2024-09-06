from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        """
        T(n) = O(n)
        S(n) = O(1)
        """
        max_profit = 0
        left, right = 0, 1
        while right < len(prices):
            if prices[left] < prices[right]:  # profit case
                profit = prices[right] - prices[left]
                max_profit = max(max_profit, profit)
            else:  # left >= right means right is the smallest
                left = right
            right += 1
        return max_profit

    def maxProfitBruteForce(self, prices: List[int]) -> int:
        """
        T(n) = O(n^2)
        S(n) = O(1)

        Brute force O(n^2)
        Iterate from first to second last, iterate from second to last
        """
        max_profit = 0
        for i in range(len(prices) - 1):
            for j in range(i + 1, len(prices)):
                max_profit = max(prices[j] - prices[i], max_profit)

        return max_profit
