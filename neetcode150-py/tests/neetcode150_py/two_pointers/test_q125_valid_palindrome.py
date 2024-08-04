from neetcode150_py.two_pointers.q125_valid_palindrome import Solution


def test_valid_palindrome() -> None:
    solution = Solution()
    assert solution.isPalindrome("A man, a plan, a canal: Panama") is True
    assert solution.isPalindrome("race a car") is False
    assert solution.isPalindrome(" ") is True


def test_valid_palindrome1() -> None:
    solution = Solution()
    assert solution.isPalindrome1("A man, a plan, a canal: Panama") is True
    assert solution.isPalindrome1("race a car") is False
    assert solution.isPalindrome1(" ") is True
