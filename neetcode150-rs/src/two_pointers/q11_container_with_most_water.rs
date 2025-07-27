pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        todo!();
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
