use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut char_count = [0; 26];
            s.chars()
                .for_each(|c| char_count[(c as usize) - ('a' as usize)] += 1);
            map.entry(char_count).or_default().push(s);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams_example1() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        let mut result = Solution::group_anagrams(input);
        result.iter_mut().for_each(|v| v.sort());
        result.sort();

        let mut expected = expected;
        expected.iter_mut().for_each(|v| v.sort());
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_anagrams_example2() {
        let input = vec!["".to_string()];
        let expected = vec![vec!["".to_string()]];

        let mut result = Solution::group_anagrams(input);
        result.iter_mut().for_each(|v| v.sort());
        result.sort();

        let mut expected = expected;
        expected.iter_mut().for_each(|v| v.sort());
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_anagrams_example3() {
        let input = vec!["a".to_string()];
        let expected = vec![vec!["a".to_string()]];

        let mut result = Solution::group_anagrams(input);
        result.iter_mut().for_each(|v| v.sort());
        result.sort();

        let mut expected = expected;
        expected.iter_mut().for_each(|v| v.sort());
        expected.sort();

        assert_eq!(result, expected);
    }
}
