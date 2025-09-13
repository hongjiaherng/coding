#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q110_balanced_binary_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,9,20,null,null,15,7]
    // Output: true
    #[test]
    fn test_is_balanced_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    // Example 2:
    // Input: root = [1,2,2,3,3,null,null,4,4]
    // Output: false
    #[test]
    fn test_is_balanced_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert_eq!(Solution::is_balanced(root), false);
    }

    // Example 3:
    // Input: root = []
    // Output: true
    #[test]
    fn test_is_balanced_example_3() {
        let root = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::is_balanced(root), true);
    }
}
