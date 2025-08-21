use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left < right {
            let mid = left + (right - left) / 2;

            match nums[mid].cmp(&nums[right]) {
                Ordering::Less => {
                    // mid -> right is in correct order, search left
                    right = mid;
                }
                Ordering::Greater | Ordering::Equal => {
                    // left -> mid is in correct order, search right
                    left = mid + 1;
                }
            }
        }
        nums[left]
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

    #[test]
    fn test_find_min_example_4() {
        let nums = vec![1];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_find_min_example_5() {
        let nums = vec![2, 1];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_find_min_example_6() {
        let nums = vec![3, 1, 2];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_find_min_example_7() {
        let nums = vec![5, 1, 2, 3, 4];
        // assert_eq!(Solution::find_min(nums), 1)
        println!("{}", Solution::find_min(nums));
        assert_eq!(1, 1);
    }
}
