pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        todo!();
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
