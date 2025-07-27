pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // 1 pass, T(n) = O(n), S(n) = O(n)
        let mut out: Vec<i32> = vec![0; temperatures.len()];
        let mut idx_stack: Vec<usize> = Vec::new();

        for (i, &temp) in temperatures.iter().enumerate() {
            while let Some(&j) = idx_stack.last() {
                if temp <= temperatures[j] {
                    break;
                }
                idx_stack.pop();
                out[j] = (i - j) as i32;
            }
            idx_stack.push(i);
        }
        out
    }

    pub fn daily_temperatures_brute_force(temperatures: Vec<i32>) -> Vec<i32> {
        let mut days_to_wait: Vec<i32> = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            for j in (i + 1)..temperatures.len() {
                if temperatures[i] < temperatures[j] {
                    days_to_wait[i] = (j - i) as i32;
                    break;
                }
            }
        }
        days_to_wait
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::q739_daily_temperatures::Solution;

    /// Example 1:
    /// Input: temperatures = [73,74,75,71,69,72,76,73]
    /// Output: [1,1,4,2,1,1,0,0]
    #[test]
    fn test_daily_temperatures_example_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }

    /// Example 2:
    /// Input: temperatures = [30,40,50,60]
    /// Output: [1,1,1,0]
    #[test]
    fn test_daily_temperatures_example_2() {
        let temperatures = vec![30, 40, 50, 60];
        let expected = vec![1, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }

    /// Example 3:
    /// Input: temperatures = [30,60,90]
    /// Output: [1,1,0]
    #[test]
    fn test_daily_temperatures_example_3() {
        let temperatures = vec![30, 60, 90];
        let expected = vec![1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }
}
