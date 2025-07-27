pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::q167_two_sum_ii_input_array_is_sorted::Solution;

    /// Example 1:
    /// Input: numbers = [2,7,11,15], target = 9
    /// Output: [1,2]
    /// Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2. We return [1, 2].
    #[test]
    fn test_two_sum_example_1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
    }

    /// Example 2:
    /// Input: numbers = [2,3,4], target = 6
    /// Output: [1,3]
    /// Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
    #[test]
    fn test_two_sum_example_2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 3]);
    }

    /// Example 3:
    /// Input: numbers = [-1,0], target = -1
    /// Output: [1,2]
    /// Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
    #[test]
    fn test_two_sum_example_3() {
        let numbers = vec![-1, 0];
        let target = -1;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
    }
}
