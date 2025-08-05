pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens.iter().rev() {
            if stack.is_empty() {
                stack.push(token.clone());
            } else {
                let parsed = token.parse::<i32>();
                if parsed.is_ok() {
                    let mut potential_lhs = parsed.ok().unwrap();
                    while !stack.is_empty() && stack.last().unwrap().parse::<i32>().is_ok() {
                        let lhs = potential_lhs;
                        let rhs = stack.pop().unwrap().parse::<i32>().ok().unwrap();
                        let op = stack.pop().unwrap();

                        potential_lhs = match op.as_str() {
                            "+" => lhs + rhs,
                            "-" => lhs - rhs,
                            "*" => lhs * rhs,
                            "/" => lhs / rhs,
                            _ => panic!("Unsupported op: {}", op),
                        };
                        println!("{} = {} {} {}", potential_lhs, lhs, op, rhs);
                    }
                    stack.push(potential_lhs.to_string());
                } else {
                    stack.push(token.clone())
                }
            }
            println!("{:?}", stack);
        }
        let ans = stack.pop().unwrap().parse::<i32>().ok().unwrap();
        ans
    }

    pub fn eval_rpn_optimal(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs + rhs);
                }
                "-" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs - rhs);
                }
                "*" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs * rhs);
                }
                "/" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs / rhs);
                }
                _ => stack.push(token.parse::<i32>().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::q150_evaluate_reverse_polish_notation::Solution;

    /// Input: tokens = ["2","1","+","3","*"]
    /// Output: 9
    /// Explanation: ((2 + 1) * 3) = 9
    #[test]
    fn test_eval_rpn1() {
        assert_eq!(
            9,
            Solution::eval_rpn(
                vec!["2", "1", "+", "3", "*"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }

    /// Input: tokens = ["4","13","5","/","+"]
    /// Output: 6
    /// Explanation: (4 + (13 / 5)) = 6
    #[test]
    fn test_eval_rpn2() {
        assert_eq!(
            6,
            Solution::eval_rpn(
                vec!["4", "13", "5", "/", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }

    /// Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
    /// Output: 22
    /// Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
    /// = ((10 * (6 / (12 * -11))) + 17) + 5
    /// = ((10 * (6 / -132)) + 17) + 5
    /// = ((10 * 0) + 17) + 5
    /// = (0 + 17) + 5
    /// = 17 + 5
    /// = 22    
    #[test]
    fn test_eval_rpn3() {
        assert_eq!(
            22,
            Solution::eval_rpn(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn test_eval_rpn_optimal1() {
        assert_eq!(
            9,
            Solution::eval_rpn_optimal(
                vec!["2", "1", "+", "3", "*"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn test_eval_rpn_optimal2() {
        assert_eq!(
            6,
            Solution::eval_rpn_optimal(
                vec!["4", "13", "5", "/", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn test_eval_rpn_optimal3() {
        assert_eq!(
            22,
            Solution::eval_rpn_optimal(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }
}
