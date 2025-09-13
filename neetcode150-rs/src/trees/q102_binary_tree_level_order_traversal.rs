#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q102_binary_tree_level_order_traversal::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,9,20,null,null,15,7]
    // Output: [[3],[9,20],[15,7]]
    #[test]
    fn test_level_order_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), expected);
    }

    // Example 2:
    // Input: root = [1]
    // Output: [[1]]
    #[test]
    fn test_level_order_example_2() {
        let root = TreeNode::from_vec(vec![Some(1)]);
        let expected = vec![vec![1]];
        assert_eq!(Solution::level_order(root), expected);
    }

    // Example 3:
    // Input: root = []
    // Output: []
    #[test]
    fn test_level_order_example_3() {
        let root = TreeNode::from_vec(vec![]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), expected);
    }
}
