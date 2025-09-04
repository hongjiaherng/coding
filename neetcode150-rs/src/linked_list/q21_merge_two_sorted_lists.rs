#![allow(unused)]
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q21_merge_two_sorted_lists::Solution;
    use crate::linked_list::ListNode;

    // Input: list1 = [1,2,4], list2 = [1,3,4]
    // Output: [1,1,2,3,4,4]
    #[test]
    fn test_merge_two_lists() {
        let list1 = ListNode::from_vec(vec![1, 2, 4]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        let expected = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    // Input: list1 = [], list2 = []
    // Output: []
    #[test]
    fn test_merge_two_lists_empty() {
        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![]);
        let expected = ListNode::from_vec(vec![]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    // Input: list1 = [], list2 = [0]
    // Output: [0]
    #[test]
    fn test_merge_two_lists_single() {
        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![0]);
        let expected = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }
}
