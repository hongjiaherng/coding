#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q100_same_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: p = [1,2,3], q = [1,2,3]
    // Output: true
    #[test]
    fn test_is_same_tree_example_1() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert!(Solution::is_same_tree(p, q));
    }

    // Example 2:
    // Input: p = [1,2], q = [1,null,2]
    // Output: false
    #[test]
    fn test_is_same_tree_example_2() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2)]);
        let q = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }

    // Example 3:
    // Input: p = [1,2,1], q = [1,1,2]
    // Output: false
    #[test]
    fn test_is_same_tree_example_3() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(1)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(1), Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }
}
