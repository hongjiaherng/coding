#![allow(unused)]
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q143_reorder_list::Solution;
    use crate::linked_list::ListNode;

    // Input: head = [1,2,3,4]
    // Output: [1,4,2,3]
    #[test]
    fn test_reorder_list() {
        let mut input = ListNode::from_vec(vec![1, 2, 3, 4]);
        let mut expected = ListNode::from_vec(vec![1, 4, 2, 3]);
        Solution::reorder_list(&mut input);
        assert_eq!(input, expected);
    }

    // Input: head = [1,2,3,4,5]
    // Output: [1,5,2,4,3]
    #[test]
    fn test_reorder_list_with_odd_length() {
        let mut input = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let mut expected = ListNode::from_vec(vec![1, 5, 2, 4, 3]);
        Solution::reorder_list(&mut input);
        assert_eq!(input, expected);
    }
}
