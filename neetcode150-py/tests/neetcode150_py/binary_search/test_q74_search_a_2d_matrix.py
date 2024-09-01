from neetcode150_py.binary_search.q74_search_a_2d_matrix import Solution


def test_search_matrix() -> None:
    solution = Solution()
    assert (
        solution.searchMatrix(
            matrix=[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], target=3
        )
        is True
    )
    assert (
        solution.searchMatrix(
            matrix=[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], target=13
        )
        is False
    )
    assert solution.searchMatrix(matrix=[[1]], target=2) is False
    assert (
        solution.searchMatrix(
            matrix=[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], target=3
        )
        is True
    )


def test_search_matrix_transform() -> None:
    solution = Solution()
    assert (
        solution.searchMatrixTransform(
            matrix=[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], target=3
        )
        is True
    )
    assert (
        solution.searchMatrixTransform(
            matrix=[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], target=13
        )
        is False
    )
    assert solution.searchMatrixTransform(matrix=[[1]], target=2) is False
    assert (
        solution.searchMatrixTransform(
            matrix=[[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], target=3
        )
        is True
    )
