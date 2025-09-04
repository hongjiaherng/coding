#![allow(unused)]
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q19_remove_nth_node_from_end_of_list::Solution;
    use crate::linked_list::ListNode;

    // Input: head = [1,2,3,4,5], n = 2
    // Output: [1,2,3,5]
    #[test]
    fn test_remove_nth_from_end() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(vec![1, 2, 3, 5]);
        assert_eq!(Solution::remove_nth_from_end(head, 2), expected);
    }

    // Input: head = [1], n = 1
    // Output: []
    #[test]
    fn test_remove_nth_from_end_single() {
        let head = ListNode::from_vec(vec![1]);
        let expected = ListNode::from_vec(vec![]);
        assert_eq!(Solution::remove_nth_from_end(head, 1), expected);
    }

    // Input: head = [1,2], n = 1
    // Output: [1]
    #[test]
    fn test_remove_nth_from_end_two() {
        let head = ListNode::from_vec(vec![1, 2]);
        let expected = ListNode::from_vec(vec![1]);
        assert_eq!(Solution::remove_nth_from_end(head, 1), expected);
    }
}
