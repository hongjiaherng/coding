from neetcode150_py.two_pointers.q15_3sum import Solution


def test_three_sum() -> None:
    solution = Solution()
    assert sorted(solution.threeSum([-1, 0, 1, 2, -1, -4])) == sorted(
        [[-1, -1, 2], [-1, 0, 1]]
    )
    assert solution.threeSum([0, 1, 1]) == []
    assert solution.threeSum([0, 0, 0]) == [[0, 0, 0]]
    assert solution.threeSum([-2, 0, 0, 2, 2]) == [[-2, 0, 2]]
    assert solution.threeSum([-1, 0, 1, 2, -1, -4]) == [[-1, -1, 2], [-1, 0, 1]]
