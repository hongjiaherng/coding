from neetcode150_py.binary_search.q33_search_in_rotated_sorted_array import Solution


def test_search() -> None:
    sol = Solution()
    assert sol.search(nums=[4, 5, 6, 7, 0, 1, 2], target=0) == 4
    assert sol.search(nums=[4, 5, 6, 7, 0, 1, 2], target=3) == -1
    assert sol.search(nums=[4, 5, 6, 7, 0, 1, 2], target=5) == 1
    assert sol.search(nums=[4, 5, 6, 7, 0, 1, 2], target=2) == 6
    assert sol.search(nums=[1], target=0) == -1
