#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q3_longest_substring_without_repeating_characters::Solution;

    #[test]
    fn test_length_of_longest_substring_example_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test_length_of_longest_substring_example_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test_length_of_longest_substring_example_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test_length_of_longest_substring_empty() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
