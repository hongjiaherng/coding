#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q199_binary_tree_right_side_view::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [1,2,3,null,5,null,4]
    // Output: [1,3,4]
    #[test]
    fn test_right_side_view_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4),
        ]);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    // Example 2:
    // Input: root = [1,2,3,4,null,null,null,5]
    // Output: [1,3,4,5]
    #[test]
    fn test_right_side_view_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4, 5]);
    }

    // Example 3:
    // Input: root = [1,null,3]
    // Output: [1,3]
    #[test]
    fn test_right_side_view_example_3() {
        let root = TreeNode::from_vec(vec![Some(1), None, Some(3)]);
        assert_eq!(Solution::right_side_view(root), vec![1, 3]);
    }

    // Example 4:
    // Input: root = []
    // Output: []
    #[test]
    fn test_right_side_view_example_4() {
        let root = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::right_side_view(root), vec![]);
    }
}
