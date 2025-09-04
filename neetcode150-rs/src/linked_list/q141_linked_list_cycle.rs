#![allow(unused)]
use crate::linked_list::ListNodeRc as ListNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        todo!()
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
}
