from neetcode150_py.arrays_and_hashing.q49_group_anagrams import Solution


def test_group_anagrams() -> None:
    solution = Solution()
    assert sorted(
        solution.groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"])
    ) == sorted(
        [
            ["eat", "tea", "ate"],
            ["tan", "nat"],
            ["bat"],
        ]
    )
    assert solution.groupAnagrams([]) == []
    assert solution.groupAnagrams(["a"]) == [["a"]]
