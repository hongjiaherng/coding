pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q424_longest_repeating_character_replacement::Solution;

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
