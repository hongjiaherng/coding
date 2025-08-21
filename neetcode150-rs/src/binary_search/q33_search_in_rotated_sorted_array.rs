pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0 as i32, (nums.len() - 1) as i32);

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] <= nums[right as usize] {
                // M -> R is in sorted order
                if nums[mid as usize] <= target && target <= nums[right as usize] {
                    // target is within M -> R
                    left = mid + 1;
                } else {
                    // out of range, search left portion
                    right = mid - 1;
                }
            } else {
                // L -> M is in sorted order
                if nums[left as usize] <= target && target <= nums[mid as usize] {
                    // target is witihin L -> M
                    right = mid - 1;
                } else {
                    // out of range, search right portion
                    left = mid + 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::q33_search_in_rotated_sorted_array::Solution;

    #[test]
    fn test_search_example_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn test_search_example_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_search_example_3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_search_example_4() {
        let nums = vec![1, 3];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_search_example_5() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 1;
        assert_eq!(Solution::search(nums, target), 5);
    }
}
