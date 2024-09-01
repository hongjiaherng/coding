from neetcode150_py.binary_search.q875_koko_eating_bananas import Solution


def test_min_eating_speed() -> None:
    solution = Solution()
    assert solution.minEatingSpeed(piles=[3, 6, 7, 11], h=8) == 4
    assert solution.minEatingSpeed(piles=[30, 11, 23, 4, 20], h=5) == 30
    assert solution.minEatingSpeed(piles=[30, 11, 23, 4, 20], h=6) == 23
    assert solution.minEatingSpeed(piles=[312884470], h=312884469) == 2
