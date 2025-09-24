use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Find depth of left subtree and right subtree then add them together
        fn height(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            match node {
                Some(node_rc) => {
                    let left = height(node_rc.borrow().left.clone(), diameter);
                    let right = height(node_rc.borrow().right.clone(), diameter);
                    *diameter = (*diameter).max(left + right);
                    1 + left.max(right)
                }
                None => 0,
            }
        }

        let mut diameter = 0;
        height(root, &mut diameter);
        diameter
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

    #[test]
    fn test_diameter_of_binary_tree_example_3() {
        let root = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 2);
    }

    #[test]
    fn test_diameter_of_binary_tree_example_4() {
        let root = TreeNode::from_vec(vec![
            Some(4),
            Some(-7),
            Some(-3),
            None,
            None,
            Some(-9),
            Some(-3),
            Some(9),
            Some(-7),
            Some(-4),
            None,
            Some(6),
            None,
            Some(-6),
            Some(-6),
            None,
            None,
            Some(0),
            Some(6),
            Some(5),
            None,
            Some(9),
            None,
            None,
            Some(-1),
            Some(-4),
            None,
            None,
            None,
            Some(-2),
        ]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 8);
    }
}
