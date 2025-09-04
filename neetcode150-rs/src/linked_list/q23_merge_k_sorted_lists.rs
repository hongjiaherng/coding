#![allow(unused)]
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q23_merge_k_sorted_lists::Solution;
    use crate::linked_list::ListNode;

    // Input: lists = [[1,4,5],[1,3,4],[2,6]]
    // Output: [1,1,2,3,4,4,5,6]
    // Explanation: The linked-lists are:
    // [
    //   1->4->5,
    //   1->3->4,
    //   2->6
    // ]
    // merging them into one sorted linked list:
    // 1->1->2->3->4->4->5->6
    #[test]
    fn test_merge_k_lists() {
        let list1 = ListNode::from_vec(vec![1, 4, 5]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        let list3 = ListNode::from_vec(vec![2, 6]);
        let merged = Solution::merge_k_lists(vec![list1, list2, list3]);
        assert_eq!(merged, ListNode::from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]));
    }

    // Input: lists = []
    // Output: []
    #[test]
    fn test_merge_k_lists_empty() {
        let merged = Solution::merge_k_lists(vec![]);
        assert_eq!(merged, None);
    }

    // Input: lists = [[]]
    // Output: []
    #[test]
    fn test_merge_k_lists_single_empty() {
        let merged = Solution::merge_k_lists(vec![None]);
        assert_eq!(merged, None);
    }
}
