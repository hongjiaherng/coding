#![allow(unused)]
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q25_reverse_nodes_in_k_group::Solution;
    use crate::linked_list::ListNode;

    // Input: head = [1,2,3,4,5], k = 2
    // Output: [2,1,4,3,5]
    #[test]
    fn test_reverse_k_group_2() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(vec![2, 1, 4, 3, 5]);
        assert_eq!(Solution::reverse_k_group(head, 2), expected);
    }

    // Input: head = [1,2,3,4,5], k = 3
    // Output: [3,2,1,4,5]
    #[test]
    fn test_reverse_k_group_3() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(vec![3, 2, 1, 4, 5]);
        assert_eq!(Solution::reverse_k_group(head, 3), expected);
    }
}
