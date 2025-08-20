use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (n_rows, n_cols) = (matrix.len() as i32, matrix[0].len() as i32);
        let mut l: i32 = 0;
        let mut r: i32 = n_rows * n_cols - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let m_x = m / n_cols;
            let m_y = m % n_cols;

            match target.cmp(&matrix[m_x as usize][m_y as usize]) {
                Ordering::Equal => return true,
                Ordering::Less => r = m - 1,
                Ordering::Greater => l = m + 1,
            }
        }

        false
    }

    pub fn search_matrix_2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (n_rows, n_cols) = (matrix.len() as i32, matrix[0].len() as i32);
        let (mut low, mut high) = (0, n_rows - 1);

        // Find the row that might contain the target
        while low <= high {
            let mid_row = low + (high - low) / 2;

            if &target < matrix[mid_row as usize].first().unwrap() {
                high = mid_row - 1;
            } else if &target > matrix[mid_row as usize].last().unwrap() {
                low = mid_row + 1;
            } else {
                // Found the row
                let target_row = &matrix[mid_row as usize];

                // Search for the target in this row
                let (mut left, mut right) = (0, n_cols - 1);
                while left <= right {
                    let mid_col = left + (right - left) / 2;

                    match target.cmp(&target_row[mid_col as usize]) {
                        Ordering::Less => right = mid_col - 1,
                        Ordering::Greater => left = mid_col + 1,
                        Ordering::Equal => return true,
                    }
                }

                // Also didn't find the target in the row
                return false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::q74_search_a_2d_matrix::Solution;

    /// Example 1:
    /// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
    /// Output: true
    #[test]
    fn test_search_matrix_example_1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert_eq!(Solution::search_matrix(matrix, target), true);
    }

    /// Example 2:
    /// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
    /// Output: false
    #[test]
    fn test_search_matrix_example_2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert_eq!(Solution::search_matrix(matrix, target), false);
    }

    #[test]
    fn test_search_matrix_example_3() {
        let matrix = vec![vec![1]];
        let target = 2;
        assert_eq!(Solution::search_matrix(matrix, target), false);
    }

    #[test]
    fn test_search_matrix_example_4() {
        let matrix = vec![vec![1, 1]];
        let target = 2;
        assert_eq!(Solution::search_matrix(matrix, target), false);
    }

    #[test]
    fn test_search_matrix_2_example_1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert_eq!(Solution::search_matrix_2(matrix, target), true);
    }

    #[test]
    fn test_search_matrix_2_example_2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert_eq!(Solution::search_matrix_2(matrix, target), false);
    }

    #[test]
    fn test_search_matrix_2_example_3() {
        let matrix = vec![vec![1]];
        let target = 2;
        assert_eq!(Solution::search_matrix_2(matrix, target), false);
    }

    #[test]
    fn test_search_matrix_2_example_4() {
        let matrix = vec![vec![1, 1]];
        let target = 2;
        assert_eq!(Solution::search_matrix_2(matrix, target), false);
    }
}
