#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q239_sliding_window_maximum::Solution;

    #[test]
    fn test_max_sliding_window_example1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let expected = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }

    #[test]
    fn test_max_sliding_window_example2() {
        let nums = vec![1];
        let k = 1;
        let expected = vec![1];
        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }
}
