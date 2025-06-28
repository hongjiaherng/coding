from typing import Set


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        substr: Set[str] = {s[0]}
        max_len = len(substr)
        left, right = 0, 1
        while right < len(s):
            if s[right] in substr:
                substr.remove(s[left])
                left += 1
            else:
                substr.add(s[right])
                max_len = max(max_len, len(substr))
                right += 1

        raise NotImplementedError()
