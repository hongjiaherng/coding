pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::q84_largest_rectangle_in_histogram::Solution;

    /// Example 1:
    /// Input: heights = [2,1,5,6,2,3]
    /// Output: 10
    /// Explanation: The above is a histogram where width of each bar is 1.
    /// The largest rectangle is shown in the red area, which has an area = 10 units.
    #[test]
    fn test_largest_rectangle_area_example_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(Solution::largest_rectangle_area(heights), 10);
    }

    /// Example 2:
    /// Input: heights = [2,4]
    /// Output: 4
    #[test]
    fn test_largest_rectangle_area_example_2() {
        let heights = vec![2, 4];
        assert_eq!(Solution::largest_rectangle_area(heights), 4);
    }
}
