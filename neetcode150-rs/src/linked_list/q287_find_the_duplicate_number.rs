#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q287_find_the_duplicate_number::Solution;

    // Input: nums = [1,3,4,2,2]
    // Output: 2
    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    // Input: nums = [3,1,3,4,2]
    // Output: 3
    #[test]
    fn test_example_2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    // Input: nums = [3,3,3,3,3]
    // Output: 3
    #[test]
    fn test_example_3() {
        let nums = vec![3, 3, 3, 3, 3];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }
}
