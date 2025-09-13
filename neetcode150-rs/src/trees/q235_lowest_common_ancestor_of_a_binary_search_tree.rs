#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q235_lowest_common_ancestor_of_a_binary_search_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
    // Output: 6
    // Explanation: The LCA of nodes 2 and 8 is 6.
    #[test]
    fn test_lowest_common_ancestor_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = TreeNode::from_vec(vec![Some(2)]);
        let q = TreeNode::from_vec(vec![Some(8)]);
        let expected = TreeNode::from_vec(vec![Some(6)]);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }

    // Example 2:
    // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
    // Output: 2
    // Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.
    #[test]
    fn test_lowest_common_ancestor_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = TreeNode::from_vec(vec![Some(2)]);
        let q = TreeNode::from_vec(vec![Some(4)]);
        let expected = TreeNode::from_vec(vec![Some(2)]);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }

    // Example 3:
    // Input: root = [2,1], p = 2, q = 1
    // Output: 2
    #[test]
    fn test_lowest_common_ancestor_example_3() {
        let root = TreeNode::from_vec(vec![Some(2), Some(1)]);
        let p = TreeNode::from_vec(vec![Some(2)]);
        let q = TreeNode::from_vec(vec![Some(1)]);
        let expected = TreeNode::from_vec(vec![Some(2)]);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }
}
