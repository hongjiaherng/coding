from neetcode150_py.stack.q84_largest_rectangle_in_histogram import Solution


def test_largest_rectangle_area() -> None:
    sol = Solution()
    assert sol.largestRectangleArea([2, 1, 5, 6, 2, 3]) == 10
    assert sol.largestRectangleArea([2, 4]) == 4


def test_largest_rectangle_area_quadratic() -> None:
    sol = Solution()
    assert sol.largestRectangleAreaQuadratic([2, 1, 5, 6, 2, 3]) == 10
    assert sol.largestRectangleAreaQuadratic([2, 4]) == 4
