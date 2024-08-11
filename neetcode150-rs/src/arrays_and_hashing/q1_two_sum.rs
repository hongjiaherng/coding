use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// - T(n) = O(n)
    /// - S(n) = O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            // Check if map contains target - num, if yes, return, otherwise, insert value
            let diff = target - num;
            if let Some(&j) = map.get(&diff) {
                return vec![i as i32, j as i32];
            }
            map.insert(num, i);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::sorted;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            sorted(Solution::two_sum(vec![2, 7, 11, 15], 9)).collect::<Vec<i32>>(),
            vec![0, 1]
        );
        assert_eq!(
            sorted(Solution::two_sum(vec![3, 2, 4], 6)).collect::<Vec<i32>>(),
            vec![1, 2]
        );
        assert_eq!(
            sorted(Solution::two_sum(vec![3, 3], 6)).collect::<Vec<i32>>(),
            vec![0, 1]
        );
    }
}
