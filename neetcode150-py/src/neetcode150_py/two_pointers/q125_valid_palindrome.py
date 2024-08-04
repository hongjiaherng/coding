class Solution:
    def isPalindrome(self, s: str) -> bool:
        """
        T(n) = O(n)
        S(n) = O(1)
        """
        left, right = 0, len(s) - 1
        while left < right:
            if not s[left].isalnum():
                left += 1
                continue
            if not s[right].isalnum():
                right -= 1
                continue
            if s[left].lower() != s[right].lower():
                return False
            left += 1
            right -= 1

        return True

    def isPalindrome1(self, s: str) -> bool:
        """
        T(n) = O(n)
        S(n) = O(n)
        """
        chars = "".join(x.lower() for x in s if x.isalnum())
        for i in range(len(chars) // 2):
            if chars[i] != chars[len(chars) - 1 - i]:
                return False
        return True
