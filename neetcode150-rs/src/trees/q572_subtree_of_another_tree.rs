use crate::trees::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let Some(sub_root_rc) = sub_root else {
            return true; // empty subtree
        };

        let mut deque = VecDeque::new();
        deque.push_back(root);

        while let Some(node_opt) = deque.pop_front() {
            if let Some(node_rc) = node_opt {
                let (val, left, right) = {
                    let node_ref = node_rc.borrow();
                    (node_ref.val, node_ref.left.clone(), node_ref.right.clone())
                };

                if val == sub_root_rc.borrow().val
                    && Self::is_same_subtree(&Some(node_rc.clone()), &Some(sub_root_rc.clone()))
                {
                    return true;
                }

                deque.push_back(left);
                deque.push_back(right);
            }
        }

        false
    }

    pub fn is_subtree_recursive_dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (_, None) => true,
            (None, Some(_)) => false,
            (Some(root_rc), Some(sub_root_rc)) => {
                let (val_root, left, right) = {
                    let root_ref = root_rc.borrow();
                    (root_ref.val, root_ref.left.clone(), root_ref.right.clone())
                };
                let val_sub_root = { sub_root_rc.borrow().val };

                if val_root == val_sub_root
                    && Self::is_same_subtree(&Some(root_rc.clone()), &Some(sub_root_rc.clone()))
                {
                    return true;
                }

                Self::is_subtree_recursive_dfs(left, Some(sub_root_rc.clone()))
                    || Self::is_subtree_recursive_dfs(right, Some(sub_root_rc.clone()))
            }
        }
    }

    fn is_same_subtree(
        a: &Option<Rc<RefCell<TreeNode>>>,
        b: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(rc_a), Some(rc_b)) => {
                let ref_a = rc_a.borrow();
                let ref_b = rc_b.borrow();

                ref_a.val == ref_b.val
                    && Self::is_same_subtree(&ref_a.left, &ref_b.left)
                    && Self::is_same_subtree(&ref_a.right, &ref_b.right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q572_subtree_of_another_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,4,5,1,2], subRoot = [4,1,2]
    // Output: true
    #[test]
    fn test_is_subtree_example_1() {
        let root = TreeNode::from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root = TreeNode::from_vec(vec![Some(4), Some(1), Some(2)]);
        assert!(Solution::is_subtree(root, sub_root));
    }

    // Example 2:
    // Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
    // Output: false
    #[test]
    fn test_is_subtree_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub_root = TreeNode::from_vec(vec![Some(4), Some(1), Some(2)]);
        assert!(!Solution::is_subtree(root, sub_root));
    }

    // Example 1:
    // Input: root = [3,4,5,1,2], subRoot = [4,1,2]
    // Output: true
    #[test]
    fn test_is_subtree_recursive_dfs_example_1() {
        let root = TreeNode::from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root = TreeNode::from_vec(vec![Some(4), Some(1), Some(2)]);
        assert!(Solution::is_subtree_recursive_dfs(root, sub_root));
    }

    // Example 2:
    // Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
    // Output: false
    #[test]
    fn test_is_subtree_recursive_dfs_example_2() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub_root = TreeNode::from_vec(vec![Some(4), Some(1), Some(2)]);
        assert!(!Solution::is_subtree_recursive_dfs(root, sub_root));
    }
}
