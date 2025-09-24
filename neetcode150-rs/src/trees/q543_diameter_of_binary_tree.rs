use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_rc) = root {
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q543_diameter_of_binary_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [1,2,3,4,5]
    // Output: 3
    // Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
    #[test]
    fn test_diameter_of_binary_tree_example_1() {
        let root = TreeNode::from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    // Example 2:
    // Input: root = [1,2]
    // Output: 1
    #[test]
    fn test_diameter_of_binary_tree_example_2() {
        let root = TreeNode::from_vec(vec![Some(1), Some(2)]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 1);
    }
}
