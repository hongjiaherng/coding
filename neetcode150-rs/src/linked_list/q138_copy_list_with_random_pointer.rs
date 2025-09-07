use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: None,
        }
    }

    pub fn from_vec(vec: Vec<(i32, Option<usize>)>) -> Option<Rc<RefCell<Node>>> {
        // First create all nodes without wiring
        let nodes: Vec<Rc<RefCell<Node>>> = vec
            .iter()
            .map(|&(val, _)| Rc::new(RefCell::new(Node::new(val))))
            .collect();

        // Then wire them together
        for i in 0..nodes.len() - 1 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }

        // Wire up random pointers
        for (i, &(_, random_index)) in vec.iter().enumerate() {
            if let Some(index) = random_index {
                nodes[i].borrow_mut().random = Some(Rc::clone(&nodes[index]));
            }
        }

        Some(Rc::clone(&nodes[0]))
    }

    pub fn to_vec(head: Option<Rc<RefCell<Node>>>) -> Vec<(i32, Option<usize>)> {
        let mut result = Vec::new();
        let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut index_map: HashMap<*const Node, usize> = HashMap::new();

        // First pass: collect nodes in order and assign indices
        let mut cur = head.clone();
        let mut idx = 0;
        while let Some(node_rc) = cur {
            let ptr: *const Node = &*node_rc.borrow();
            index_map.insert(ptr, idx);
            nodes.push(Rc::clone(&node_rc));
            cur = node_rc.borrow().next.clone();
            idx += 1;
        }

        // Second pass: build Vec<(val, random_index)>
        for node_rc in nodes {
            let node_ref = node_rc.borrow();
            let val = node_ref.val;

            let random_index = node_ref.random.as_ref().map(|r| {
                let ptr: *const Node = &*r.borrow();
                index_map[&ptr]
            });

            result.push((val, random_index));
        }

        result
    }
}

pub struct Solution;

impl Solution {
    pub fn copy_random_list(head: &Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let Some(head_rc) = head else {
            return None;
        };

        let mut old2copy: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();

        // First pass: Build a hashmap of old node ptr to new copied node
        let mut curr_opt = Some(head_rc.clone());
        while let Some(curr_rc) = curr_opt {
            let curr_ptr = Rc::as_ptr(&curr_rc);
            let curr_ref = curr_rc.borrow();
            let copy_rc = Rc::new(RefCell::new(Node::new(curr_ref.val)));

            old2copy.insert(curr_ptr, copy_rc);
            curr_opt = curr_ref.next.clone();
        }

        // Second pass, connect the new copied linked list with next ptr and random ptr
        curr_opt = Some(head_rc.clone());
        while let Some(curr_rc) = curr_opt {
            let curr_ref = curr_rc.borrow();
            let curr_ptr = Rc::as_ptr(&curr_rc);

            if let Some(copy_rc) = old2copy.get(&curr_ptr) {
                let mut copy_ref = copy_rc.borrow_mut();

                copy_ref.next = curr_ref
                    .next
                    .as_ref()
                    .map(|next_rc| old2copy[&Rc::as_ptr(next_rc)].clone());

                copy_ref.random = curr_ref
                    .random
                    .as_ref()
                    .map(|random_rc| old2copy[&Rc::as_ptr(random_rc)].clone());
            }

            curr_opt = curr_ref.next.clone();
        }

        old2copy.get(&Rc::as_ptr(head_rc)).cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q138_copy_list_with_random_pointer::{Node, Solution};

    // Input: head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
    // Output: [[7,null],[13,0],[11,4],[10,2],[1,0]]
    #[test]
    fn test_copy_random_list() {
        let head = Node::from_vec(vec![
            (7, None),
            (13, Some(0)),
            (11, Some(4)),
            (10, Some(2)),
            (1, Some(0)),
        ]);
        let copied = Solution::copy_random_list(&head);
        assert_eq!(Node::to_vec(copied), Node::to_vec(head));
    }

    // Input: head = [[1,1],[2,1]]
    // Output: [[1,1],[2,1]]
    #[test]
    fn test_copy_random_list_2() {
        let head = Node::from_vec(vec![(1, Some(1)), (2, Some(1))]);
        let copied = Solution::copy_random_list(&head);
        assert_eq!(Node::to_vec(copied), Node::to_vec(head));
    }

    // Input: head = [[3,null],[3,0],[3,null]]
    // Output: [[3,null],[3,0],[3,null]]
    #[test]
    fn test_copy_random_list_3() {
        let head = Node::from_vec(vec![(3, None), (3, Some(0)), (3, None)]);
        let copied = Solution::copy_random_list(&head);
        assert_eq!(Node::to_vec(copied), Node::to_vec(head));
    }
}
