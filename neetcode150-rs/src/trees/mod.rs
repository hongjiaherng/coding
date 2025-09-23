use std::cell::RefCell;
use std::rc::Rc;

pub mod q100_same_tree;
pub mod q102_binary_tree_level_order_traversal;
pub mod q104_maximum_depth_of_binary_tree;
pub mod q105_construct_binary_tree_from_preorder_and_inorder_traversal;
pub mod q110_balanced_binary_tree;
pub mod q124_binary_tree_maximum_path_sum;
pub mod q1448_count_good_nodes_in_binary_tree;
pub mod q199_binary_tree_right_side_view;
pub mod q226_invert_binary_tree;
pub mod q230_kth_smallest_element_in_a_bst;
pub mod q235_lowest_common_ancestor_of_a_binary_search_tree;
pub mod q297_serialize_and_deserialize_binary_tree;
pub mod q543_diameter_of_binary_tree;
pub mod q572_subtree_of_another_tree;
pub mod q98_validate_binary_search_tree;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v_iter = v.into_iter();
        match v_iter.next().flatten() {
            Some(root_val) => {
                let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
                let mut deque = std::collections::VecDeque::new();
                deque.push_back(root.clone());

                while let Some(node) = deque.pop_front() {
                    if let Some(left_val) = v_iter.next().flatten() {
                        let left = Rc::new(RefCell::new(TreeNode::new(left_val)));
                        node.borrow_mut().left = Some(left.clone());
                        deque.push_back(left);
                    }
                    if let Some(right_val) = v_iter.next().flatten() {
                        let right = Rc::new(RefCell::new(TreeNode::new(right_val)));
                        node.borrow_mut().right = Some(right.clone());
                        deque.push_back(right);
                    }
                }
                Some(root)
            }
            None => None,
        }
    }
}
