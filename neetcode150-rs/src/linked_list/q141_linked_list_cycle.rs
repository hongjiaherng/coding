use crate::linked_list::ListNodeRc as ListNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut visited: HashSet<*mut ListNode> = HashSet::new();
        let mut curr = head;

        while let Some(node) = curr.take() {
            let ptr = node.as_ptr();
            if visited.contains(&ptr) {
                return true;
            }
            visited.insert(ptr);
            curr = node.borrow().next.clone();
        }

        false
    }

    pub fn has_cycle_floyd(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = head.clone();
        let mut fast = head;

        while let (Some(slow_node), Some(fast_node)) = (slow.take(), fast.take()) {
            slow = slow_node.borrow().next.clone();

            if let Some(fast_next) = fast_node.borrow().next.clone() {
                fast = fast_next.borrow().next.clone();
            } else {
                return false;
            }

            if let (Some(s), Some(f)) = (&slow, &fast) {
                if Rc::ptr_eq(s, f) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::linked_list::q141_linked_list_cycle::Solution;
    use crate::linked_list::ListNodeRc as ListNode;

    // Input: head = [3,2,0,-4], pos = 1
    // Output: true
    // Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).
    #[test]
    fn test_has_cycle_example_1() {
        let head = Rc::new(RefCell::new(ListNode::new(3)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));
        let node3 = Rc::new(RefCell::new(ListNode::new(0)));
        let node4 = Rc::new(RefCell::new(ListNode::new(-4)));

        head.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));
        node3.borrow_mut().next = Some(Rc::clone(&node4));
        node4.borrow_mut().next = Some(Rc::clone(&node2));

        assert!(Solution::has_cycle(Some(head)));
    }

    // Input: head = [1,2], pos = 0
    // Output: true
    // Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
    #[test]
    fn test_has_cycle_example_2() {
        let head = Rc::new(RefCell::new(ListNode::new(1)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));

        head.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&head));

        assert!(Solution::has_cycle(Some(head)));
    }

    // Input: head = [1], pos = -1
    // Output: false
    // Explanation: There is no cycle in the linked list.
    #[test]
    fn test_has_cycle_example_3() {
        let head = Rc::new(RefCell::new(ListNode::new(1)));

        assert!(!Solution::has_cycle(Some(head)));
    }

    // Input: head = [3,2,0,-4], pos = 1
    // Output: true
    // Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).
    #[test]
    fn test_has_cycle_floyd_example_1() {
        let head = Rc::new(RefCell::new(ListNode::new(3)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));
        let node3 = Rc::new(RefCell::new(ListNode::new(0)));
        let node4 = Rc::new(RefCell::new(ListNode::new(-4)));

        head.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));
        node3.borrow_mut().next = Some(Rc::clone(&node4));
        node4.borrow_mut().next = Some(Rc::clone(&node2));

        assert!(Solution::has_cycle_floyd(Some(head)));
    }

    // Input: head = [1,2], pos = 0
    // Output: true
    // Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
    #[test]
    fn test_has_cycle_floyd_example_2() {
        let head = Rc::new(RefCell::new(ListNode::new(1)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));

        head.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&head));

        assert!(Solution::has_cycle_floyd(Some(head)));
    }

    // Input: head = [1], pos = -1
    // Output: false
    // Explanation: There is no cycle in the linked list.
    #[test]
    fn test_has_cycle_floyd_example_3() {
        let head = Rc::new(RefCell::new(ListNode::new(1)));

        assert!(!Solution::has_cycle_floyd(Some(head)));
    }
}
