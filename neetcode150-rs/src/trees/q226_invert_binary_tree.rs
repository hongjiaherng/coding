use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc) = &root {
            let mut node = node_rc.borrow_mut();

            let left = node.left.take();
            let right = node.right.take();

            node.left = Self::invert_tree(right);
            node.right = Self::invert_tree(left);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q226_invert_binary_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [4,2,7,1,3,6,9]
    // Output: [4,7,2,9,6,3,1]
    //
    //        4
    //       / \
    //      2   7
    //     / \ / \
    //    1  3 6  9
    //
    #[test]
    fn test_invert_tree_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let expected = TreeNode::from_vec(vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);
        assert_eq!(Solution::invert_tree(root), expected);
    }

    // Example 2:
    // Input: root = [2,1,3]
    // Output: [2,3,1]
    #[test]
    fn test_invert_tree_example_2() {
        let root = TreeNode::from_vec(vec![Some(2), Some(1), Some(3)]);
        let expected = TreeNode::from_vec(vec![Some(2), Some(3), Some(1)]);
        assert_eq!(Solution::invert_tree(root), expected);
    }

    // Example 3:
    // Input: root = []
    // Output: []
    #[test]
    fn test_invert_tree_example_3() {
        let root = TreeNode::from_vec(vec![]);
        let expected = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::invert_tree(root), expected);
    }
}
