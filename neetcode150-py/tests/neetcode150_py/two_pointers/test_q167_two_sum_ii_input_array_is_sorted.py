from neetcode150_py.two_pointers.q167_two_sum_ii_input_array_is_sorted import Solution


def test_two_sum() -> None:
    solution = Solution()
    assert solution.twoSum([2, 7, 11, 15], 9) == [1, 2]
    assert solution.twoSum([2, 3, 4], 6) == [1, 3]
    assert solution.twoSum([-1, 0], -1) == [1, 2]
    assert solution.twoSum([2, 7, 11, 15], 26) == [3, 4]
