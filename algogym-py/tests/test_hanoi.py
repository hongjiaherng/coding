import pytest

from algogym_py.hanoi import Solution


@pytest.mark.parametrize(
    "n_disks, expected_moves", [(1, 1), (2, 3), (3, 7), (4, 15), (5, 31)]
)
def test_hanoi(n_disks: int, expected_moves: int):
    n_moves = Solution().solve_hanoi(n_disks)
    assert n_moves == expected_moves
