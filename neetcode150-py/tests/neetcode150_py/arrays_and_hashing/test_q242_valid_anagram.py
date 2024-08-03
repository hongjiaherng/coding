from neetcode150_py.arrays_and_hashing.q242_valid_anagram import Solution


def test_valid_anagram() -> None:
    solution = Solution()
    assert solution.isAnagram("anagram", "nagaram") is True
    assert solution.isAnagram("rat", "car") is False
