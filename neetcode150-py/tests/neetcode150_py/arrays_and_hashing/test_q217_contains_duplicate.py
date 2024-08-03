from neetcode150_py.arrays_and_hashing.q217_contains_duplicate import Solution


def test_contains_duplicate() -> None:
    solution = Solution()

    assert solution.containsDuplicate([1, 2, 3, 1]) is True
    assert solution.containsDuplicate([1, 2, 3, 4]) is False
    assert solution.containsDuplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2]) is True
