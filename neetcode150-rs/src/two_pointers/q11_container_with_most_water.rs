use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            let area = min(height[left], height[right]) * (right - left) as i32;
            max_area = max(max_area, area);
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::q11_container_with_most_water::Solution;

    /// Example 1:
    /// Input: height = [1,8,6,2,5,4,8,3,7]
    /// Output: 49
    /// Explanation: The above is a graphical representation of the array.
    /// The maximum area of water that can be contained is 49.
    #[test]
    fn test_max_area_example_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }

    /// Example 2:
    /// Input: height = [1,1]
    /// Output: 1
    #[test]
    fn test_max_area_example_2() {
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
