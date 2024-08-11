pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
