use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums_set: HashSet<i32> = HashSet::new();

        for num in nums {
            if !nums_set.insert(num) {
                return true;
            }
        }
        false
    }

    pub fn contains_duplicate_slow(nums: Vec<i32>) -> bool {
        let vec_len = nums.len();
        let nums_set: HashSet<i32> = HashSet::from_iter(nums);
        vec_len != nums_set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate_example1() {
        let input = vec![1, 2, 3, 1];
        let expected = true;
        let result = Solution::contains_duplicate(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_contains_duplicate_example2() {
        let input = vec![1, 2, 3, 4];
        let expected = false;
        let result = Solution::contains_duplicate(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_contains_duplicate_example3() {
        let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let expected = true;
        let result = Solution::contains_duplicate(input);

        assert_eq!(result, expected);
    }
}
