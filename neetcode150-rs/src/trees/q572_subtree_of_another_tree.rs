#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q572_subtree_of_another_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,4,5,1,2], subRoot = [4,1,2]
    // Output: true
    #[test]
    fn test_is_subtree_example_1() {
        let root = TreeNode::from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root = TreeNode::from_vec(vec![Some(4), Some(1), Some(2)]);
        assert!(Solution::is_subtree(root, sub_root));
    }

    // Example 2:
    // Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
    // Output: false
    #[test]
    fn test_is_subtree_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub_root = TreeNode::from_vec(vec![Some(4), Some(1), Some(2)]);
        assert!(!Solution::is_subtree(root, sub_root));
    }
}
