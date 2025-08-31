pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }

        let (mut count_t, mut count_s) = ([0usize; 128], [0usize; 128]);

        let mut required = 0;
        for &ch in t.as_bytes() {
            if count_t[ch as usize] == 0 {
                required += 1;
            }
            count_t[ch as usize] += 1;
        }

        let mut best_len = usize::MAX;
        let mut best_start = 0;
        let mut matches = 0;
        let chars = s.as_bytes();
        let mut l = 0;

        for r in 0..chars.len() {
            let idx_r = chars[r] as usize;
            count_s[idx_r] += 1;

            if count_s[idx_r] == count_t[idx_r] && count_t[idx_r] > 0 {
                matches += 1;
            }

            // Everything in count_s is complete match or greater than those in count_t
            while matches == required {
                // Get the shortest matching substring
                if best_len > r - l + 1 {
                    best_len = r - l + 1;
                    best_start = l;
                }

                let idx_l = chars[l] as usize;
                count_s[idx_l] -= 1;
                // If shifting window to left causes condition to not match
                if count_s[idx_l] < count_t[idx_l] && count_t[idx_l] > 0 {
                    matches -= 1;
                }

                l += 1;
            }
        }

        if best_len == usize::MAX {
            String::new()
        } else {
            String::from_utf8(chars[best_start..best_start + best_len].to_vec()).unwrap()
        }
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
