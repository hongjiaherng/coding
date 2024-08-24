from neetcode150_py.stack.q22_generate_parentheses import Solution


def test_generate_parenthesis() -> None:
    solution = Solution()
    assert solution.generateParenthesis(3) == ["((()))","(()())","(())()","()(())","()()()"]
    assert solution.generateParenthesis(1) == ["()"]
