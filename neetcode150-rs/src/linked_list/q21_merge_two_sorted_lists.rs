use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let val1 = list1.as_ref().unwrap().val;
            let val2 = list2.as_ref().unwrap().val;

            if val1 <= val2 {
                let mut node = list1.take().unwrap();
                list1 = node.next.take(); // Shift list1 forward
                curr.next = Some(node);
            } else {
                let mut node = list2.take().unwrap();
                list2 = node.next.take();
                curr.next = Some(node);
            }

            // Shift curr pointer forward
            curr = curr.next.as_mut().unwrap();
        }

        // Attach remaining nodes
        curr.next = list1.or(list2);

        dummy.next
    }

    pub fn merge_two_lists_recursive(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val <= node2.val {
                    node1.next = Self::merge_two_lists_recursive(node1.next.take(), Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_two_lists_recursive(Some(node1), node2.next.take());
                    Some(node2)
                }
            }
        }
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

    // Input: list1 = [1,2,4], list2 = [1,3,4]
    // Output: [1,1,2,3,4,4]
    #[test]
    fn test_merge_two_lists_recursive() {
        let list1 = ListNode::from_vec(vec![1, 2, 4]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        let expected = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists_recursive(list1, list2), expected);
    }

    // Input: list1 = [], list2 = []
    // Output: []
    #[test]
    fn test_merge_two_lists_recursive_empty() {
        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![]);
        let expected = ListNode::from_vec(vec![]);
        assert_eq!(Solution::merge_two_lists_recursive(list1, list2), expected);
    }

    // Input: list1 = [], list2 = [0]
    // Output: [0]
    #[test]
    fn test_merge_two_lists_recursive_single() {
        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![0]);
        let expected = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::merge_two_lists_recursive(list1, list2), expected);
    }
}
