use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count_map: HashMap<char, i32> = HashMap::new();

        s.chars()
            .for_each(|c| *count_map.entry(c).or_insert(0) += 1);

        println!("{:?}", count_map);
        for c in t.chars() {
            let count = count_map.get_mut(&c);
            
            // let mut count = count_map.get(&c).unwrap_or(&0);
            // if *count == 0 {
            //     return false;
            // }
            // count -= 1;
            println!("{:?}", count);
        }
        println!("{:?} {}", count_map, s);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_example1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let expected = true;
        let result = Solution::is_anagram(s, t);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_anagram_example2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let expected = false;
        let result = Solution::is_anagram(s, t);

        assert_eq!(result, expected);
    }
}
