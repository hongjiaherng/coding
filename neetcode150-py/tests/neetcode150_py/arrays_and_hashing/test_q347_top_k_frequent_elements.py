from neetcode150_py.arrays_and_hashing.q347_top_k_frequent_elements import Solution


def test_top_k_frequent_elements() -> None:
    solution = Solution()

    assert solution.topKFrequent([1, 1, 1, 2, 2, 3], 2) == [1, 2]
    assert solution.topKFrequent([1], 1) == [1]
    assert solution.topKFrequent([1, 2, 3], 10) == [1, 2, 3]


def test_top_k_frequent_elements1() -> None:
    solution = Solution()

    assert solution.topKFrequent1([1, 1, 1, 2, 2, 3], 2) == [1, 2]
    assert solution.topKFrequent1([1], 1) == [1]
