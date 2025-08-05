pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        strs.into_iter()
            .map(|s| format!("{}#{}", s.len(), s))
            .collect()
    }

    pub fn decode(s: String) -> Vec<String> {
        // Look for digit#
        let mut decoded: Vec<String> = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = s.chars().collect();

        while i < chars.len() {
            let mut j = i;
            while chars[j] != '#' {
                j += 1;
            }

            let wordlen: usize = chars[i..j].iter().collect::<String>().parse().unwrap();
            j += 1; // skip the '#'

            decoded.push(chars[j..j + wordlen].iter().collect());
            i = j + wordlen;
        }
        decoded
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::q271_encode_and_decode_strings::Solution;

    #[test]
    fn test_encode_and_decode() {
        let strs = vec!["hello".to_string(), "world".to_string()];
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_empty_strings() {
        let strs = vec!["".to_string(), "test".to_string()];
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_example1() {
        let strs: Vec<String> = vec!["neet", "code", "love", "you"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_example2() {
        let strs: Vec<String> = vec!["we", "say", ":", "yes"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let encoded = Solution::encode(strs.clone());
        let decoded = Solution::decode(encoded);
        assert_eq!(decoded, strs);
    }
}
