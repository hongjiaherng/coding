from typing import List


class Solution:
    """
    Description:
    - Exactly 1 solution
    - Not allowed to use the same element twice

    Constraint:
    - S(n) = O(1)

    """

    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        """
        T(n) = O(n)
        S(n) = O(1)
        """
        left, right = 0, len(numbers) - 1
        while left < right:
            lr_sum = numbers[left] + numbers[right]
            if lr_sum == target:
                break
            elif lr_sum < target:
                left += 1
            else:
                right -= 1

        return [left + 1, right + 1]
