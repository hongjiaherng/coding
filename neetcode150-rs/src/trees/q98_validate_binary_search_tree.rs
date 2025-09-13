#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q98_validate_binary_search_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [2,1,3]
    // Output: true
    // Example 2:
    #[test]
    fn test_is_valid_bst_example_1() {
        let root = TreeNode::from_vec(vec![Some(2), Some(1), Some(3)]);
        assert!(Solution::is_valid_bst(root));
    }

    // Example 2:
    // Input: root = [5,1,4,null,null,3,6]
    // Output: false
    // Explanation: The root node's value is 5 but its right child's value is 4.
    #[test]
    fn test_is_valid_bst_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ]);
        assert!(!Solution::is_valid_bst(root));
    }
}
