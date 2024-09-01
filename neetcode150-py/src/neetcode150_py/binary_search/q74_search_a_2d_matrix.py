from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        """
        T(n) = O(log (m * n))
        """
        top = 0
        bottom = len(matrix) - 1

        while top <= bottom:
            midrow = top + (bottom - top) // 2
            if matrix[midrow][-1] < target:
                top = midrow + 1
            elif matrix[midrow][0] > target:
                bottom = midrow - 1
            else:
                # matrix[midrow][0] <= target <= matrix[midrow][-1]:
                left = 0
                right = len(matrix[midrow]) - 1
                while left <= right:
                    midcol = left + (right - left) // 2
                    if matrix[midrow][midcol] == target:
                        return True
                    elif matrix[midrow][midcol] < target:
                        left = midcol + 1
                    else:
                        right = midcol - 1
                return False

        return False

    def searchMatrixTransform(self, matrix: List[List[int]], target: int) -> bool:
        """
        T(n) = O(log (m * n))
        """
        left = 0
        right = len(matrix) * len(matrix[0]) - 1

        while left <= right:
            mid = left + (right - left) // 2
            mid_i = mid // len(matrix[0])
            mid_j = mid % len(matrix[0])

            if matrix[mid_i][mid_j] == target:
                return True
            elif matrix[mid_i][mid_j] < target:
                left = mid + 1
            else:
                right = mid - 1

        return False
