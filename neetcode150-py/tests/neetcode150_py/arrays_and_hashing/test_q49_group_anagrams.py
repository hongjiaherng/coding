from neetcode150_py.arrays_and_hashing.q49_group_anagrams import Solution


def test_group_anagrams() -> None:
    solution = Solution()
    assert solution.groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]) == [
        ["eat", "tea", "ate"],
        ["tan", "nat"],
        ["bat"],
    ]


def test_empty_list() -> None:
    solution = Solution()
    assert solution.groupAnagrams([]) == []


def test_single_element() -> None:
    solution = Solution()
    assert solution.groupAnagrams(["a"]) == [["a"]]
