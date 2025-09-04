use std::rc::Rc;
use std::{cell::RefCell, fmt};

pub mod q138_copy_list_with_random_pointer;
pub mod q141_linked_list_cycle;
pub mod q143_reorder_list;
pub mod q146_lru_cache;
pub mod q19_remove_nth_node_from_end_of_list;
pub mod q206_reverse_linked_list;
pub mod q21_merge_two_sorted_lists;
pub mod q23_merge_k_sorted_lists;
pub mod q25_reverse_nodes_in_k_group;
pub mod q287_find_the_duplicate_number;
pub mod q2_add_two_numbers;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    pub fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            vec.push(node.val);
            curr = node.next.as_ref();
        }
        vec
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = Some(Box::new(self.clone()));
        let vec = ListNode::to_vec(&head);
        write!(f, "{:?}", vec)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNodeRc {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNodeRc>>>,
}

impl ListNodeRc {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNodeRc { val, next: None }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Rc<RefCell<ListNodeRc>>> {
        let mut head = None;
        for &v in vec.iter().rev() {
            let mut node = ListNodeRc::new(v);
            node.next = head;
            head = Some(Rc::new(RefCell::new(node)));
        }
        head
    }
}
