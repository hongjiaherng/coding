import pytest

from algogym_py.greedy_remove_adj_duplicates import Solution

# solution = Solution()
# assert solution.recursive_remove_duplicates("abcd") == "abcd"
# assert solution.recursive_remove_duplicates("abcccbabc") == "bc"
# assert solution.recursive_remove_duplicates("deeedbbcccbdaa") == "bd"
# assert solution.recursive_remove_duplicates("caaabbbaacdddd") == ""
# assert solution.recursive_remove_duplicates("acaaabbbacdddd") == "acac"
# assert solution.recursive_remove_duplicates("abcdefggfedcba") == ""

# assert solution.stack_remove_duplicates("abcd") == "abcd"
# assert solution.stack_remove_duplicates("abcccbabc") == "bc"
# assert solution.stack_remove_duplicates("abcccccbabc") == "bc"
# assert solution.stack_remove_duplicates("deeedbbcccbdaa") == "bd"
# assert solution.stack_remove_duplicates("caaabbbaacdddd") == ""
# assert solution.stack_remove_duplicates("acaaabbbacdddd") == "acac"
# assert solution.stack_remove_duplicates("abcdefggfedcba") == ""

# assert solution.crazy_remove_duplicates("abcd") == "abcd"
# assert solution.crazy_remove_duplicates("abcccbabc") == "bc"
# assert solution.crazy_remove_duplicates("abcccccbabc") == "bc"
# assert solution.crazy_remove_duplicates("deeedbbcccbdaa") == "bd"
# assert solution.crazy_remove_duplicates("caaabbbaacdddd") == ""
# assert solution.crazy_remove_duplicates("acaaabbbacdddd") == "acac"
# assert solution.crazy_remove_duplicates("abcdefggfedcba") == ""


@pytest.mark.parametrize(
    "s, expected",
    [
        ("abcd", "abcd"),
        ("abcccbabc", "bc"),
        ("deeedbbcccbdaa", "bd"),
        ("caaabbbaacdddd", ""),
        ("acaaabbbacdddd", "acac"),
        ("abcdefggfedcba", ""),
    ],
)
def test_greedy_remove_adj_duplicates(s: str, expected: str):
    solution = Solution()
    assert solution.recursive_remove_duplicates(s) == expected
    assert solution.stack_remove_duplicates(s) == expected
    assert solution.crazy_remove_duplicates(s) == expected
