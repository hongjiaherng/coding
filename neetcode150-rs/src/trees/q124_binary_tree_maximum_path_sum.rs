#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q124_binary_tree_maximum_path_sum::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [1,2,3]
    // Output: 6
    // Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
    #[test]
    fn test_max_path_sum_example_1() {
        let root = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::max_path_sum(root), 6);
    }

    // Example 2:
    // Input: root = [-10,9,20,null,null,15,7]
    // Output: 42
    // Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
    #[test]
    fn test_max_path_sum_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_path_sum(root), 42);
    }
}
