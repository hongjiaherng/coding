use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();

        let mut triplets = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let three_sum = nums[left] + nums[right] + nums[i];
                if three_sum == 0 {
                    triplets.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    // Skip duplicates for the left pointer
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                } else if three_sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        triplets
    }

    pub fn three_sum_hashset(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut triplets: HashSet<Vec<i32>> = HashSet::new();

        let mut nums = nums;
        nums.sort_unstable();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let target = -nums[i];
            let mut map: HashMap<i32, usize> = HashMap::new();

            for j in i + 1..nums.len() {
                let diff = target - nums[j];
                if let Some(&_) = map.get(&diff) {
                    triplets.insert(vec![nums[i], diff, nums[j]]);
                }
                map.insert(nums[j], j);
            }
        }

        triplets.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::q15_3sum::Solution;

    /// Example 1:
    /// Input: nums = [-1,0,1,2,-1,-4]
    /// Output: [[-1,-1,2],[-1,0,1]]
    /// Explanation:
    /// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
    /// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
    /// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
    /// The distinct triplets are [-1,0,1] and [-1,-1,2].
    /// Notice that the order of the output and the order of the triplets does not matter.
    #[test]
    fn test_three_sum_example_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = Solution::three_sum(nums);
        result.iter_mut().for_each(|e| e.sort());
        result.sort();

        let mut expected: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.iter_mut().for_each(|e| e.sort());
        expected.sort();
        assert_eq!(result, expected);
    }

    /// Example 2:
    /// Input: nums = [0,1,1]
    /// Output: []
    /// Explanation: The only possible triplet does not sum up to 0.
    #[test]
    fn test_three_sum_example_2() {
        let nums = vec![0, 1, 1];
        let result = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    /// Example 3:
    /// Input: nums = [0,0,0]
    /// Output: [[0,0,0]]
    /// Explanation: The only possible triplet sums up to 0.
    #[test]
    fn test_three_sum_example_3() {
        let nums = vec![0, 0, 0];
        let result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_three_sum_example_4() {
        let nums = vec![0, 0, 0, 0];
        let result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![0, 0, 0]])
    }

    #[test]
    fn test_three_sum_example_5() {
        let nums = vec![-2, 0, 1, 1, 2];
        let result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![-2, 0, 2], vec![-2, 1, 1]]);
    }

    #[test]
    fn test_three_sum_example_6() {
        let nums = vec![2, -3, 0, -2, -5, -5, -4, 1, 2, -2, 2, 0, 2, -4, 5, 5, -10];
        let mut result = Solution::three_sum(nums);
        let mut expected: Vec<Vec<i32>> = vec![
            vec![-10, 5, 5],
            vec![-5, 0, 5],
            vec![-4, 2, 2],
            vec![-3, -2, 5],
            vec![-3, 1, 2],
            vec![-2, 0, 2],
        ];

        result.iter_mut().for_each(|e| e.sort());
        expected.iter_mut().for_each(|e| e.sort());
        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }
}
