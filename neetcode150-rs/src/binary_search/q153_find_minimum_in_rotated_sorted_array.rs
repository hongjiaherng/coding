pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::q153_find_minimum_in_rotated_sorted_array::Solution;

    #[test]
    fn test_find_min() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_find_min_example_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::find_min(nums), 0);
    }

    #[test]
    fn test_find_min_example_3() {
        let nums = vec![11, 13, 15, 17];
        assert_eq!(Solution::find_min(nums), 11);
    }
}
