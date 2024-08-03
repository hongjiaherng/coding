from neetcode150_py.arrays_and_hashing.q347_top_k_frequent_elements import Solution


def test_top_k_frequent_elements() -> None:
    solution = Solution()

    assert solution.topKFrequent([1, 1, 1, 2, 2, 3], 2) == [1, 2]
    assert solution.topKFrequent([1], 1) == [1]
