from neetcode150_py.arrays_and_hashing.q238_product_of_array_except_self import Solution


def test_product_of_array_except_self() -> None:
    solution = Solution()
    assert solution.productExceptSelf([1, 2, 3, 4]) == [24, 12, 8, 6]
    assert solution.productExceptSelf([-1, 1, 0, -3, 3]) == [0, 0, 9, 0, 0]


def test_product_of_array_except_self2() -> None:
    solution = Solution()
    assert solution.productExceptSelf2([1, 2, 3, 4]) == [24, 12, 8, 6]
    assert solution.productExceptSelf2([-1, 1, 0, -3, 3]) == [0, 0, 9, 0, 0]


def test_product_of_array_except_self1() -> None:
    solution = Solution()
    assert solution.productExceptSelf1([1, 2, 3, 4]) == [24, 12, 8, 6]
    assert solution.productExceptSelf1([-1, 1, 0, -3, 3]) == [0, 0, 9, 0, 0]
