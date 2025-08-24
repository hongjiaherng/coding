use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// T(n) = O(n)
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut longest = 0;
        let mut left = 0;
        let mut count: HashMap<char, i32> = HashMap::new();
        let mut max_count = 0;

        for right in 0..chars.len() {
            *count.entry(chars[right]).or_insert(0) += 1;
            max_count = max_count.max(*count.get(&chars[right]).unwrap());

            while (right - left + 1) as i32 - max_count > k {
                *count.get_mut(&chars[left]).unwrap() -= 1;
                left += 1;
            }

            longest = longest.max((right - left + 1) as i32);
        }
        longest
    }

    /// T(n) = O(26n)
    pub fn character_replacement_standard(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut count: HashMap<char, i32> = HashMap::new();
        let mut longest = 0;
        let mut left = 0;

        for right in 0..chars.len() {
            *count.entry(chars[right]).or_insert(0) += 1;
            let max_count = *count.values().max().unwrap(); // O(26)

            while (right - left + 1) as i32 - max_count > k {
                *count.get_mut(&chars[left]).unwrap() -= 1;
                left += 1;
            }

            longest = longest.max((right - left + 1) as i32);
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q424_longest_repeating_character_replacement::Solution;

    #[test]
    fn test_character_replacement_standard_example1() {
        let s = "ABAB".to_string();
        let k = 2;
        assert_eq!(Solution::character_replacement_standard(s, k), 4);
    }

    #[test]
    fn test_character_replacement_standard_example2() {
        let s = "AABABBA".to_string();
        let k = 1;
        assert_eq!(Solution::character_replacement_standard(s, k), 4);
    }

    #[test]
    fn test_character_replacement_example1() {
        let s = "ABAB".to_string();
        let k = 2;
        assert_eq!(Solution::character_replacement(s, k), 4);
    }

    #[test]
    fn test_character_replacement_example2() {
        let s = "AABABBA".to_string();
        let k = 1;
        assert_eq!(Solution::character_replacement(s, k), 4);
    }
}
