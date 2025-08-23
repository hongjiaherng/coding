use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut seen = HashSet::new();
        let mut max_len = 0;
        let mut left = 0;

        for right in 0..chars.len() {
            while seen.contains(&chars[right]) {
                seen.remove(&chars[left]);
                left += 1;
            }
            seen.insert(chars[right]);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
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

    #[test]
    fn test_length_of_longest_substring_example_4() {
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }
}
