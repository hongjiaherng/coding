use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());

        if m > n {
            return false;
        }

        let (mut count1, mut count2) = ([0; 26], [0; 26]);
        let (s1_bytes, s2_bytes) = (s1.as_bytes(), s2.as_bytes());

        for i in 0..m {
            count1[(s1_bytes[i] - b'a') as usize] += 1;
            count2[(s2_bytes[i] - b'a') as usize] += 1;
        }

        let mut matches = (0..26).filter(|&i| count1[i] == count2[i]).count();

        for l in 1..=n - m {
            if matches == 26 {
                return true;
            }

            let r = l + m - 1;
            let (idx_add, idx_remove) = (
                (s2_bytes[r] - b'a') as usize,
                (s2_bytes[l - 1] - b'a') as usize,
            );

            // Add new char at r
            count2[idx_add] += 1;
            if count2[idx_add] == count1[idx_add] {
                // if adding new char cause this char to match, increment matches
                matches += 1;
            } else if count2[idx_add] - 1 == count1[idx_add] {
                // if adding new char deviates the original match by 1, decrement matches
                matches -= 1;
            }

            // Remove char at l - 1
            count2[idx_remove] -= 1;
            if count2[idx_remove] == count1[idx_remove] {
                matches += 1;
            } else if count2[idx_remove] + 1 == count1[idx_remove] {
                matches -= 1;
            }
        }

        matches == 26
    }

    pub fn check_inclusion_standard(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());

        if m > n {
            return false;
        }

        // O(m)
        let mut s1_count: HashMap<char, i32> = HashMap::new();
        s1.chars().into_iter().for_each(|c| {
            *s1_count.entry(c).or_insert(0) += 1;
        });

        let s2_chars = s2.chars().collect::<Vec<_>>();
        let mut s2_count: HashMap<char, i32> = HashMap::new();
        (0..(m - 1)).for_each(|i| {
            *s2_count.entry(s2_chars[i]).or_insert(0) += 1;
        });

        for l in 0..n - m + 1 {
            let r = l + m - 1;

            *s2_count.entry(s2_chars[r]).or_insert(0) += 1;

            // Compare two hashmap
            let mut n_match = 0;
            for (k1, v1) in s1_count.iter() {
                if let Some(v2) = s2_count.get(k1) {
                    if v2 == v1 {
                        n_match += v1.clone() as usize;
                        continue;
                    }
                }
                break;
            }

            if n_match == m {
                return true;
            }

            *s2_count.get_mut(&s2_chars[l]).unwrap() -= 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q567_permutation_in_string::Solution;

    #[test]
    fn test_check_inclusion_example_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert!(Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_example_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert!(!Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_example_3() {
        let s1 = "horse".to_owned();
        let s2 = "ros".to_owned();
        assert!(!Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_example_4() {
        let s1 = "abcdxabcde".to_owned();
        let s2 = "abcdeabcdx".to_owned();
        assert!(Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_example_5() {
        let s1 = "a".to_owned();
        let s2 = "ab".to_owned();
        assert!(Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_example_6() {
        let s1 = "adc".to_owned();
        let s2 = "dcda".to_owned();
        assert!(Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_example_7() {
        let s1 = "hello".to_owned();
        let s2 = "ooolleoooleh".to_owned();
        assert!(!Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_example_8() {
        let s1 = "abc".to_owned();
        let s2 = "bbbca".to_owned();
        assert!(Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert!(Solution::check_inclusion_standard(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert!(!Solution::check_inclusion_standard(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_3() {
        let s1 = "horse".to_owned();
        let s2 = "ros".to_owned();
        assert!(!Solution::check_inclusion_standard(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_4() {
        let s1 = "abcdxabcde".to_owned();
        let s2 = "abcdeabcdx".to_owned();
        assert!(Solution::check_inclusion_standard(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_5() {
        let s1 = "a".to_owned();
        let s2 = "ab".to_owned();
        assert!(Solution::check_inclusion_standard(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_6() {
        let s1 = "adc".to_owned();
        let s2 = "dcda".to_owned();
        assert!(Solution::check_inclusion_standard(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_7() {
        let s1 = "hello".to_owned();
        let s2 = "ooolleoooleh".to_owned();
        assert!(!Solution::check_inclusion_standard(s1, s2));
    }

    #[test]
    fn test_check_inclusion_standard_example_8() {
        let s1 = "abc".to_owned();
        let s2 = "bbbca".to_owned();
        assert!(Solution::check_inclusion_standard(s1, s2));
    }
}
