pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::q15_3sum::Solution;

    /// Example 1:
    /// Input: nums = [-1,0,1,2,-1,-4]
    /// Output: [[-1,-1,2],[-1,0,1]]
    /// Explanation:
    /// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
    /// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
    /// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
    /// The distinct triplets are [-1,0,1] and [-1,-1,2].
    /// Notice that the order of the output and the order of the triplets does not matter.
    #[test]
    fn test_three_sum_example_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    /// Example 2:
    /// Input: nums = [0,1,1]
    /// Output: []
    /// Explanation: The only possible triplet does not sum up to 0.
    #[test]
    fn test_three_sum_example_2() {
        let nums = vec![0, 1, 1];
        let result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![]]);
    }

    /// Example 3:
    /// Input: nums = [0,0,0]
    /// Output: [[0,0,0]]
    /// Explanation: The only possible triplet sums up to 0.
    #[test]
    fn test_three_sum_example_3() {
        let nums = vec![0, 0, 0];
        let result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }
}
