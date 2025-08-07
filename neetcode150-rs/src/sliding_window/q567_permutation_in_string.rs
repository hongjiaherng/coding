pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        todo!();
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
}
