use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// - T(n) = O(n)
    /// - S(n) = O(n)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            if !set.insert(num) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
