pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Self::backtrack(&mut result, String::new(), 0, 0, n);
        result
    }

    fn backtrack(result: &mut Vec<String>, current: String, open: i32, close: i32, max: i32) {
        if open == max && close == max {
            // println!("{}", current);
            result.push(current);
            return;
        }
        if open < max {
            Self::backtrack(result, format!("{}(", current), open + 1, close, max);
        }
        if close < open {
            Self::backtrack(result, format!("{})", current), open, close + 1, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::q22_generate_parentheses::Solution;

    /// Example 1:
    /// Input: n = 3
    /// Output: ["((()))","(()())","(())()","()(())","()()()"]
    #[test]
    fn test_example_1() {
        let expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(Solution::generate_parenthesis(3), expected);
    }

    /// Example 2:
    /// Input: n = 1
    /// Output: ["()"]
    #[test]
    fn test_example_2() {
        let expected = vec!["()".to_string()];
        assert_eq!(Solution::generate_parenthesis(1), expected);
    }
}
