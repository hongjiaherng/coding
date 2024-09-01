from neetcode150_py.binary_search.q704_binary_search import Solution


def test_search() -> None:
    solution = Solution()
    assert solution.search(nums=[-1, 0, 3, 5, 9, 12], target=9) == 4
    assert solution.search(nums=[-1, 0, 3, 5, 9, 12], target=2) == -1
