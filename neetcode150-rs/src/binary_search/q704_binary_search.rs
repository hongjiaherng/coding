pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::q704_binary_search::Solution;

    /// Example 1:
    /// Input: nums = [-1,0,3,5,9,12], target = 9
    /// Output: 4
    /// Explanation: 9 exists in nums and its index is 4
    #[test]
    fn test_search_example_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(Solution::search(nums, target), 4);
    }

    /// Example 2:
    /// Input: nums = [-1,0,3,5,9,12], target = 2
    /// Output: -1
    /// Explanation: 2 does not exist in nums so return -1
    #[test]
    fn test_search_example_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
