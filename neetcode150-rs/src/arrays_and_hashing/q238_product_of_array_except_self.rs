pub struct Solution;

/**
 * Naive solution:
 * - Loop through each element, for each element, loop through the nums except self while multiplying every value iterated over, T(n) = O(n^2), S(n) = O(1)
 *
 * prefix = [1, 1*2, 1*2*3, 1*2*3*4] = [1, 2, 6, 24]
 * suffix = [1*2*3*4, 2*3*4, 3*4, 4] = [24, 24, 12, 4]
 *
 * [suffix[1], prefix[0] * suffix[2], prefix[1] * suffix[3], prefix[2]]
 *
 *
 * prefix = [a[0], a[0]*a[1], a[0]*a[1]*a[2], a[0]*a[1]*a[2]*a[3], a[0]*a[1]*a[2]*a[3]*a[4]]
 * suffix = [a[0]*a[1]*a[2]*a[3]*a[4], a[1]*a[2]*a[3]*a[4], a[2]*a[3]*a[4], a[3]*a[4], a[4]]
 *
 * [suffix[1], prefix[0] * suffix[2], prefix[1] * suffix[3], prefix[2] * suffix[4], prefix[3]]
 *
 */

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut out = vec![1; nums.len()];
        let mut prefix = 1;
        let mut suffix = 1;

        for (i, &num) in nums.iter().enumerate() {
            out[i] = prefix;
            prefix *= num;
        }

        for (i, &num) in nums.iter().enumerate().rev() {
            out[i] *= suffix;
            suffix *= num;
        }
        out
    }

    pub fn linear_space_product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = nums.clone();
        let mut suffix = nums.clone();
        let mut out = vec![1; nums.len()];

        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1] * nums[i];
        }

        for i in (0..nums.len() - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i];
        }

        for i in 0..nums.len() {
            if i != 0 {
                out[i] *= prefix[i - 1];
            }
            if i != nums.len() - 1 {
                out[i] *= suffix[i + 1];
            }
        }
        out
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

    /// Example 1:
    /// Input: nums = [1,2,3,4]
    /// Output: [24,12,8,6]
    #[test]
    fn test_linear_space_product_except_self_example_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::linear_space_product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    /// Example 2:
    /// Input: nums = [-1,1,0,-3,3]
    /// Output: [0,0,9,0,0]
    #[test]
    fn test_linear_space_product_except_self_example_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = Solution::linear_space_product_except_self(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
