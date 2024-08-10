from neetcode150_py.two_pointers.q42_trapping_rain_water import Solution


def test_trap() -> None:
    solution = Solution()
    assert solution.trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) == 6
    assert solution.trap([4, 2, 0, 3, 2, 5]) == 9
