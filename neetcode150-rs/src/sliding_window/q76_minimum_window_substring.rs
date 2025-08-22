#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q76_minimum_window_substring::Solution;

    #[test]
    fn test_min_window_example_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        assert_eq!(Solution::min_window(s, t), "BANC".to_string());
    }

    #[test]
    fn test_min_window_example_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        assert_eq!(Solution::min_window(s, t), "a".to_string());
    }

    #[test]
    fn test_min_window_example_3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        assert_eq!(Solution::min_window(s, t), "".to_string());
    }
}
