from neetcode150_py.sliding_window.q3_longest_substring_without_repeating_characters import (
    Solution,
)


def test_length_of_longest_substring() -> None:
    sol = Solution()
    assert sol.lengthOfLongestSubstring("abcabcbb") == 3
    assert sol.lengthOfLongestSubstring("bbbbb") == 1
    assert sol.lengthOfLongestSubstring("pwwkew") == 3
