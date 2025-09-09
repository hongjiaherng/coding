use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Edge case
        if lists.is_empty() {
            return None;
        }

        // Base case
        if lists.len() == 1 {
            return lists[0].take();
        }

        // Divide into subproblems
        let second = lists.split_off(lists.len() / 2);

        let merged_first = Self::merge_k_lists(lists);
        let merged_second = Self::merge_k_lists(second);

        Self::merge_lists(merged_first, merged_second)
    }

    fn merge_lists(
        mut first: Option<Box<ListNode>>,
        mut second: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy;

        while first.is_some() && second.is_some() {
            let val1 = first.as_ref().unwrap().val;
            let val2 = second.as_ref().unwrap().val;

            tail.next = if val1 <= val2 {
                let mut node = first.take().unwrap();
                first = node.next.take();
                Some(node)
            } else {
                let mut node = second.take().unwrap();
                second = node.next.take();
                Some(node)
            };

            tail = tail.next.as_mut().unwrap();
        }

        tail.next = first.or(second);

        dummy.next
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
        assert_eq!(merged, ListNode::from_vec(vec![]));
    }

    // Input: lists = [[]]
    // Output: []
    #[test]
    fn test_merge_k_lists_single_empty() {
        let list1 = ListNode::from_vec(vec![]);
        let merged = Solution::merge_k_lists(vec![list1]);
        assert_eq!(merged, ListNode::from_vec(vec![]));
    }
}
