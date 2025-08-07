pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        todo!();
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
}
