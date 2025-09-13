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
        if v.is_empty() {
            return None;
        }

        let mut queue = std::collections::VecDeque::new();
        let root = Rc::new(RefCell::new(TreeNode::new(v[0].unwrap())));
        queue.push_back(root.clone());

        let mut i = 1;
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if i < v.len() {
                if let Some(val) = v[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < v.len() {
                if let Some(val) = v[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }
}
