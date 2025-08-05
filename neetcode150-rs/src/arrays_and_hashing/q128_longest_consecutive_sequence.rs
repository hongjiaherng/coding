use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);
        let mut max_seq_len = 0;

        for &num in &set {
            if set.contains(&(num - 1)) {
                // this num is not a seq head
                continue;
            }
            // this num is a seq head, keep searching up
            let mut cur_seq_len = 1;
            while set.contains(&(num + cur_seq_len)) {
                cur_seq_len += 1;
            }

            max_seq_len = max_seq_len.max(cur_seq_len);
        }
        max_seq_len
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::q128_longest_consecutive_sequence::Solution;

    /// Example 1:
    /// Input: nums = [100,4,200,1,3,2]
    /// Output: 4
    /// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
    #[test]
    fn test_longest_consecutive_example_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    /// Example 2:
    /// Input: nums = [0,3,7,2,5,8,4,6,0,1]
    /// Output: 9
    #[test]
    fn test_longest_consecutive_example_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }

    /// Example 3:
    /// Input: nums = [1,0,1,2]
    /// Output: 3
    #[test]
    fn test_longest_consecutive_example_3() {
        let nums = vec![1, 0, 1, 2];
        assert_eq!(Solution::longest_consecutive(nums), 3);
    }
}
