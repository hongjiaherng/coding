from neetcode150_py.stack.q853_car_fleet import Solution


def test_car_fleet() -> None:
    solution = Solution()
    assert (
        solution.carFleet(target=12, position=[10, 8, 0, 5, 3], speed=[2, 4, 1, 1, 3])
        == 3
    )
    assert solution.carFleet(target=10, position=[3], speed=[3]) == 1
    assert solution.carFleet(target=100, position=[0, 2, 4], speed=[4, 2, 1]) == 1
