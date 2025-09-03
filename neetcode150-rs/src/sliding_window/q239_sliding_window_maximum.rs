use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(nums.len() - k as usize + 1);
        let mut deque: VecDeque<usize> = VecDeque::new(); // Monotonically decreasing queue

        for (i, &num) in nums.iter().enumerate() {
            // Remove first element from queue if the element is out of range of the current window (l..=r)
            if let Some(&front_i) = deque.front() {
                if i >= front_i + k as usize {
                    deque.pop_front();
                }
            }

            // Attempt to add element to the back of the queue, remove any element that's less than the current element
            while let Some(&back_i) = deque.back() {
                if num > nums[back_i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);

            // The first element is the max of the current window
            if i + 1 >= k as usize {
                res.push(nums[*deque.front().unwrap()]);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q239_sliding_window_maximum::Solution;

    #[test]
    fn test_max_sliding_window_example1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let expected = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }

    #[test]
    fn test_max_sliding_window_example2() {
        let nums = vec![1];
        let k = 1;
        let expected = vec![1];
        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }

    #[test]
    fn test_max_sliding_window_example3() {
        let nums = vec![1, -1];
        let k = 1;
        assert_eq!(Solution::max_sliding_window(nums, k), vec![1, -1]);
    }

    #[test]
    fn test_max_sliding_window_example4() {
        let nums = vec![9, 11];
        let k = 2;
        assert_eq!(Solution::max_sliding_window(nums, k), vec![11]);
    }
}
