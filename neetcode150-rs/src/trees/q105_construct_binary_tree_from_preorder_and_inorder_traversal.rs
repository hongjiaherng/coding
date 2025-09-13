#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q105_construct_binary_tree_from_preorder_and_inorder_traversal::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
    // Output: [3,9,20,null,null,15,7]
    #[test]
    pub fn test_build_tree_example_1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = Solution::build_tree(preorder, inorder);
        let expected = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(root, expected);
    }

    // Example 2:
    // Input: preorder = [-1], inorder = [-1]
    // Output: [-1]
    #[test]
    pub fn test_build_tree_example_2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let root = Solution::build_tree(preorder, inorder);
        let expected = TreeNode::from_vec(vec![Some(-1)]);
        assert_eq!(root, expected);
    }
}
