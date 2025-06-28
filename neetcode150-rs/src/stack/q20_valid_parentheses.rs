use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn my_is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mapping: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')]
            .iter()
            .cloned()
            .collect();

        for c in s.chars() {
            if mapping.contains_key(&c) {
                if stack.is_empty() || stack.pop() != Some(*mapping.get(&c).unwrap()) {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }

        stack.is_empty()
    }

    pub fn super_rusty_is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::q20_valid_parentheses::Solution;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::my_is_valid("()".to_string()), true);
        assert_eq!(Solution::my_is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::my_is_valid("(]".to_string()), false);
        assert_eq!(Solution::my_is_valid("(]".to_string()), false);
        assert_eq!(Solution::my_is_valid("([])".to_string()), true);
        assert_eq!(Solution::my_is_valid("([)]".to_string()), false);
    }

    #[test]
    fn test_super_rusty_is_valid() {
        assert_eq!(Solution::super_rusty_is_valid("()".to_string()), true);
        assert_eq!(Solution::super_rusty_is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::super_rusty_is_valid("(]".to_string()), false);
        assert_eq!(Solution::super_rusty_is_valid("(]".to_string()), false);
        assert_eq!(Solution::super_rusty_is_valid("([])".to_string()), true);
        assert_eq!(Solution::super_rusty_is_valid("([)]".to_string()), false);
        assert_eq!(Solution::super_rusty_is_valid("][)]".to_string()), false);
    }
}
