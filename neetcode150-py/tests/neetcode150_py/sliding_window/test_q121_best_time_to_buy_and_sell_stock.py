from neetcode150_py.sliding_window.q121_best_time_to_buy_and_sell_stock import Solution


def test_max_profit() -> None:
    solution = Solution()
    assert solution.maxProfit([7, 1, 5, 3, 6, 4]) == 5
    assert solution.maxProfit([7, 6, 4, 3, 1]) == 0

def test_max_profit_brute_force() -> None:
    solution = Solution()
    assert solution.maxProfitBruteForce([7, 1, 5, 3, 6, 4]) == 5
    assert solution.maxProfitBruteForce([7, 6, 4, 3, 1]) == 0
