use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// - T(n) = O(n)
    /// - S(n) = O(n)
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num_counts: HashMap<i32, usize> = HashMap::new();

        for &n in &nums {
            *num_counts.entry(n).or_default() += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
        for (&n, &count) in &num_counts {
            buckets[count - 1].push(n);
        }

        let mut res = Vec::with_capacity(k as usize);
        for bucket in buckets.iter().rev() {
            for &n in bucket {
                res.push(n);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
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
