from neetcode150_py.arrays_and_hashing.q128_longest_consecutive_sequence import Solution


def test_longest_consecutive() -> None:
    solution = Solution()
    assert solution.longestConsecutive([100, 4, 200, 1, 3, 2]) == 4
    assert solution.longestConsecutive([0, 3, 7, 2, 5, 8, 4, 6, 0, 1]) == 9
    assert solution.longestConsecutive([]) == 0
    assert solution.longestConsecutive([9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]) == 7


def test_longest_consecutive1() -> None:
    solution = Solution()
    assert solution.longestConsecutive1([100, 4, 200, 1, 3, 2]) == 4
    assert solution.longestConsecutive1([0, 3, 7, 2, 5, 8, 4, 6, 0, 1]) == 9
    assert solution.longestConsecutive1([]) == 0
    assert solution.longestConsecutive1([9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]) == 7
