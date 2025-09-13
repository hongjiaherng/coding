#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q104_maximum_depth_of_binary_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,9,20,null,null,15,7]
    // Output: 3
    #[test]
    fn test_max_depth_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }

    // Example 2:
    // Input: root = [1,null,2]
    // Output: 2
    #[test]
    fn test_max_depth_example_2() {
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(root), 2);
    }

    // Example 3:
    // Input: root = []
    // Output: 0
    #[test]
    fn test_max_depth_example_3() {
        let root = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::max_depth(root), 0);
    }
}
