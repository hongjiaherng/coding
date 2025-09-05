use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Edge case: 0 or 1 node → nothing to reorder
        if head.as_ref().and_then(|n| n.next.as_ref()).is_none() {
            return;
        }

        // Get the second half, unlink first half to second half
        let mut second = Self::split_second_half(head);

        // Reverse the second half
        Self::reverse(&mut second);

        // Merge first half & second half
        Self::merge(head, &mut second);
    }

    fn split_second_half(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow: *mut Option<Box<ListNode>> = head; // raw ptr to traverse
        let mut fast: Option<&Box<ListNode>> = head
            .as_ref()
            .and_then(|n| n.next.as_ref())
            .and_then(|n| n.next.as_ref());

        // Get the second half
        while let Some(f) = fast {
            fast = f.next.as_ref().and_then(|n| n.next.as_ref());
            unsafe {
                let slow_mut: Option<&mut Box<ListNode>> = slow.as_mut().unwrap().as_mut();
                slow = &mut slow_mut.unwrap().next;
            }
        }

        // Unlink first half to second half
        unsafe {
            let first: Option<&mut Box<ListNode>> = slow.as_mut().unwrap().as_mut(); // last node in first half
            let second: Option<Box<ListNode>> = first.unwrap().next.take(); // first node in the second half, take() clear out the value in .next and return it
            second
        }
    }

    fn reverse(head: &mut Option<Box<ListNode>>) {
        let mut prev = None;
        let mut curr = head.take();

        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        *head = prev;
    }

    fn merge(first: &mut Option<Box<ListNode>>, second: &mut Option<Box<ListNode>>) {
        let mut part1: Option<Box<ListNode>> = first.take();
        let mut part2: Option<Box<ListNode>> = second.take();

        let mut dummy: Option<Box<ListNode>> = None;
        let mut tail: &mut Option<Box<ListNode>> = &mut dummy;

        while part1.is_some() || part2.is_some() {
            if let Some(mut n1) = part1 {
                part1 = n1.next.take();
                *tail = Some(n1);
                tail = &mut tail.as_mut().unwrap().next;
            }
            if let Some(mut n2) = part2 {
                part2 = n2.next.take();
                *tail = Some(n2);
                tail = &mut tail.as_mut().unwrap().next;
            }
        }

        *first = dummy;
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
        let expected = ListNode::from_vec(vec![1, 4, 2, 3]);
        Solution::reorder_list(&mut input);
        assert_eq!(input, expected);
    }

    // Input: head = [1,2,3,4,5]
    // Output: [1,5,2,4,3]
    #[test]
    fn test_reorder_list_with_odd_length() {
        let mut input = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(vec![1, 5, 2, 4, 3]);
        Solution::reorder_list(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_reorder_list_with_empty_list() {
        let mut input = ListNode::from_vec(vec![]);
        let expected = ListNode::from_vec(vec![]);
        Solution::reorder_list(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_reorder_list_with_single_node() {
        let mut input = ListNode::from_vec(vec![1]);
        let expected = ListNode::from_vec(vec![1]);
        Solution::reorder_list(&mut input);
        assert_eq!(input, expected);
    }
}
