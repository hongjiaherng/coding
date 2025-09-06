use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Dummy node in front of head
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });

        let mut slow: *mut Option<Box<ListNode>> = &mut dummy.next;
        let mut fast = &dummy.next;

        // Move fast n steps ahead
        for _ in 0..n {
            if let Some(f) = fast {
                fast = &f.next;
            }
        }

        // Move them together till fast hit null
        while let Some(f) = fast {
            fast = &f.next;
            unsafe {
                slow = &mut (*slow).as_mut().unwrap().next;
            }
        }

        // Since fast is n steps ahead of slow, if fast is now at end, then slow is n node from the end too
        unsafe {
            if let Some(mut node) = (*slow).take() {
                *slow = node.next.take();
            }
        }
        dummy.next
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
