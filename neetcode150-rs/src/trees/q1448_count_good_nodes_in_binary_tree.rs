#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q1448_count_good_nodes_in_binary_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,1,4,3,null,1,5]
    // Output: 4
    // Explanation: Nodes in blue are good.
    // Root Node (3) is always a good node.
    // Node 4 -> (3,4) is the maximum value in the path starting from the root.
    // Node 5 -> (3,4,5) is the maximum value in the path
    // Node 3 -> (3,1,3) is the maximum value in the path.
    #[test]
    fn test_good_nodes_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(1),
            Some(4),
            Some(3),
            None,
            Some(1),
            Some(5),
        ]);
        assert_eq!(Solution::good_nodes(root), 4);
    }

    // Example 2:
    // Input: root = [3,3,null,4,2]
    // Output: 3
    // Explanation: Node 2 -> (3, 3, 2) is not good, because "3" is higher than it.
    #[test]
    fn test_good_nodes_example_2() {
        let root = TreeNode::from_vec(vec![Some(3), Some(3), None, Some(4), Some(2)]);
        assert_eq!(Solution::good_nodes(root), 3);
    }

    // Example 3:
    // Input: root = [1]
    // Output: 1
    // Explanation: Root is considered as good.
    #[test]
    fn test_good_nodes_example_3() {
        let root = TreeNode::from_vec(vec![Some(1)]);
        assert_eq!(Solution::good_nodes(root), 1);
    }
}
