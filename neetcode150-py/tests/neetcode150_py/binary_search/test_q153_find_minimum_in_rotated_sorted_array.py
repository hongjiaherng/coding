from neetcode150_py.binary_search.q153_find_minimum_in_rotated_sorted_array import Solution


def test_find_min() -> None:
    sol = Solution()
    assert sol.findMin([3,4,5,1,2]) == 1
    assert sol.findMin([4,5,6,7,0,1,2]) == 0
    assert sol.findMin([11,13,15,17]) == 11
