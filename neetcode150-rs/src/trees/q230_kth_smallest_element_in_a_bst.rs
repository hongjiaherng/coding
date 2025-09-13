#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q230_kth_smallest_element_in_a_bst::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,1,4,null,2], k = 1
    // Output: 1
    #[test]
    fn test_kth_smallest_example_1() {
        let root = TreeNode::from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]);
        assert_eq!(Solution::kth_smallest(root, 1), 1);
    }

    // Example 2:
    // Input: root = [5,3,6,2,4,null,null,1], k = 3
    // Output: 3
    #[test]
    fn test_kth_smallest_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        assert_eq!(Solution::kth_smallest(root, 3), 3);
    }
}
