from neetcode150_py.two_pointers.q11_container_with_most_water import Solution


def test_max_area() -> None:
    solution = Solution()
    assert solution.maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7]) == 49
    assert solution.maxArea([1, 1]) == 1
    assert solution.maxArea([0, 2]) == 0
    assert solution.maxArea([2,3,4,5,18,17,6]) == 17

def test_brute_force_max_area() -> None:
    solution = Solution()
    assert solution.brute_force_max_area([1, 8, 6, 2, 5, 4, 8, 3, 7]) == 49
    assert solution.brute_force_max_area([1, 1]) == 1
    assert solution.brute_force_max_area([0, 2]) == 0
    assert solution.brute_force_max_area([2,3,4,5,18,17,6]) == 17
