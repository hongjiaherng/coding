use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// - T(n) = O(n)
    /// - S(n) = O(n)
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        };

        let mut map: HashMap<char, i32> = HashMap::new();

        for (char_s, char_t) in s.chars().zip(t.chars()) {
            *map.entry(char_s).or_default() += 1;
            *map.entry(char_t).or_default() -= 1;
        }

        map.values().all(|&v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
        assert_eq!(
            Solution::is_anagram("abc".to_string(), "abcd".to_string()),
            false
        );
    }
}
