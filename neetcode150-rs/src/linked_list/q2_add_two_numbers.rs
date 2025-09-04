#![allow(unused)]
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q2_add_two_numbers::ListNode;
    use crate::linked_list::q2_add_two_numbers::Solution;

    // Input: l1 = [2,4,3], l2 = [5,6,4]
    // Output: [7,0,8]
    // Explanation: 342 + 465 = 807.
    #[test]
    fn test_add_two_numbers_example1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let expected = ListNode::from_vec(vec![7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    // Input: l1 = [0], l2 = [0]
    // Output: [0]
    #[test]
    fn test_add_two_numbers_example2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let expected = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    // Output: [8,9,9,9,0,0,0,1]
    #[test]
    fn test_add_two_numbers_example3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let expected = ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}
