use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// - T(n) = O(m * n), where m = avg length of string in strs, n = length of strs
    /// - S(n) = O(26 * n) + O(m * n) = O(m * n),  where we have n number of list of 26 elements for count array
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut key: [usize; 26] = [0; 26];
            s.chars()
                .for_each(|c| key[(c as usize) - ('a' as usize)] += 1);
            map.entry(key).or_default().push(s);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_group_anagrams() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let output: Vec<Vec<String>> = Solution::group_anagrams(input)
            .into_iter()
            .map(|v| v.into_iter().sorted().collect())
            .sorted()
            .collect();
        let expected: Vec<Vec<String>> = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ]
        .into_iter()
        .map(|v| v.into_iter().sorted().collect())
        .sorted()
        .collect();
        assert_eq!(output, expected);

        let input = vec!["".to_string()];
        let output = Solution::group_anagrams(input);
        let expected = vec![vec!["".to_string()]];
        assert_eq!(output, expected);

        let input = vec!["a".to_string()];
        let output = Solution::group_anagrams(input);
        let expected = vec![vec!["a".to_string()]];
        assert_eq!(output, expected);
    }
}
