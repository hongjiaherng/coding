pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut max_area = 0;

        for i in 0..heights.len() {
            let mut start = i;
            while !stack.is_empty() && heights[i] < stack.last().unwrap().1 {
                let (j, heights_j) = stack.pop().unwrap();
                max_area = max_area.max(heights_j * (i - j) as i32);
                start = j;
            }
            stack.push((start, heights[i]));
        }

        while let Some((j, h)) = stack.pop() {
            max_area = max_area.max(h * (heights.len() - j) as i32);
        }

        max_area
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
