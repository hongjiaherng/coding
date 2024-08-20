from neetcode150_py.stack.q20_valid_parentheses import Solution


def test_is_valid() -> None:
    solution = Solution()
    assert solution.isValid("()") is True
    assert solution.isValid("()[]{}") is True
    assert solution.isValid("(]") is False
