from neetcode150_py.arrays_and_hashing.q1_two_sum import Solution


def test_two_sum() -> None:
    solution = Solution()
    assert sorted(solution.twoSum(nums=[2, 7, 11, 15], target=9)) == [0, 1]
    assert sorted(solution.twoSum(nums=[3, 2, 4], target=6)) == [1, 2]
    assert sorted(solution.twoSum(nums=[3, 3], target=6)) == [0, 1]
    assert solution.twoSum(nums=[2, 7, 11, 15], target=10) == []

def test_two_sum1() -> None:
    solution = Solution()
    assert sorted(solution.twoSum1(nums=[2, 7, 11, 15], target=9)) == [0, 1]
    assert sorted(solution.twoSum1(nums=[3, 2, 4], target=6)) == [1, 2]
    assert sorted(solution.twoSum1(nums=[3, 3], target=6)) == [0, 1]
    assert solution.twoSum1(nums=[2, 7, 11, 15], target=10) == []
