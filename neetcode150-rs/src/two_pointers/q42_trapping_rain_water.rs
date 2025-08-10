pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut vol = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        let (mut prefix, mut suffix) = (height[l], height[r]);

        while l < r {
            if height[l] <= height[r] {
                l += 1;
                prefix = prefix.max(height[l]);
                vol += prefix - height[l];
            } else {
                r -= 1;
                suffix = suffix.max(height[r]);
                vol += suffix - height[r];
            }
        }

        vol
    }

    pub fn trap_monotonic_stack(height: Vec<i32>) -> i32 {
        let mut prefix: Vec<i32> = height.clone();
        let mut suffix: Vec<i32> = height.clone();
        let mut vol = 0;

        for i in 1..height.len() {
            prefix[i] = prefix[i - 1].max(height[i]);
        }

        for i in (0..height.len() - 1).rev() {
            suffix[i] = suffix[i + 1].max(height[i]);
        }

        for i in 0..height.len() {
            vol += prefix[i].min(suffix[i]) - height[i];
        }

        vol
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::q42_trapping_rain_water::Solution;

    /// Example 1:
    /// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
    /// Output: 6
    #[test]
    fn test_trap_example_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }

    /// Example 2:
    /// Input: height = [4,2,0,3,2,5]
    /// Output: 9
    #[test]
    fn test_trap_example_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }
}
