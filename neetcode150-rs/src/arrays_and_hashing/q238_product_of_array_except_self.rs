pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::q238_product_of_array_except_self::Solution;

    /// Example 1:
    /// Input: nums = [1,2,3,4]
    /// Output: [24,12,8,6]
    #[test]
    fn test_product_except_self_example_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    /// Example 2:
    /// Input: nums = [-1,1,0,-3,3]
    /// Output: [0,0,9,0,0]
    #[test]
    fn test_product_except_self_example_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
