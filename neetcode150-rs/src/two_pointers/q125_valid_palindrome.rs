pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Filter out non-alphanumeric characters and convert to lowercase
        let s = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();

        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut right = s.len().saturating_sub(1); // Use saturating_sub to handle small lengths

        while left < right {
            if s_bytes[left] != s_bytes[right] {
                return false;
            }
            left += 1;
            right = right.saturating_sub(1); // Use saturating_sub to handle small lengths
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
