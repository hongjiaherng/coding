use crate::trees::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // DFS
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(rc_p), Some(rc_q)) => {
                let node_p = rc_p.borrow();
                let node_q = rc_q.borrow();

                if node_p.val != node_q.val {
                    return false;
                }

                Self::is_same_tree(node_p.left.clone(), node_q.left.clone())
                    && Self::is_same_tree(node_p.right.clone(), node_q.right.clone())
            }
        }
    }

    pub fn is_same_tree_bfs(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut deque_p = VecDeque::new();
        let mut deque_q = VecDeque::new();

        deque_p.push_back(p);
        deque_q.push_back(q);

        while !deque_p.is_empty() && !deque_p.is_empty() {
            // Iterate over the current level
            for _ in 0..deque_p.len() {
                let node_p = deque_p.pop_front().flatten();
                let node_q = deque_q.pop_front().flatten();

                match (node_p, node_q) {
                    (None, None) => continue,
                    (None, Some(_)) | (Some(_), None) => return false,
                    (Some(rc_p), Some(rc_q)) => {
                        let ref_p = rc_p.borrow();
                        let ref_q = rc_q.borrow();

                        if ref_p.val != ref_q.val {
                            return false;
                        }

                        deque_p.push_back(ref_p.left.clone());
                        deque_p.push_back(ref_p.right.clone());
                        deque_q.push_back(ref_q.left.clone());
                        deque_q.push_back(ref_q.right.clone());
                    }
                }
            }
        }
        true
    }

    pub fn is_same_tree_iterative_dfs(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack = Vec::new();
        stack.push((p, q));

        while let Some((node_p, node_q)) = stack.pop() {
            match (node_p, node_q) {
                (None, None) => continue,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(rc_p), Some(rc_q)) => {
                    let ref_p = rc_p.borrow();
                    let ref_q = rc_q.borrow();

                    if ref_p.val != ref_q.val {
                        return false;
                    }

                    stack.push((ref_p.right.clone(), ref_q.right.clone()));
                    stack.push((ref_p.left.clone(), ref_q.left.clone()));
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q100_same_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: p = [1,2,3], q = [1,2,3]
    // Output: true
    #[test]
    fn test_is_same_tree_example_1() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert!(Solution::is_same_tree(p, q));
    }

    // Example 2:
    // Input: p = [1,2], q = [1,null,2]
    // Output: false
    #[test]
    fn test_is_same_tree_example_2() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2)]);
        let q = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }

    // Example 3:
    // Input: p = [1,2,1], q = [1,1,2]
    // Output: false
    #[test]
    fn test_is_same_tree_example_3() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(1)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(1), Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }

    // Example 1:
    // Input: p = [1,2,3], q = [1,2,3]
    // Output: true
    #[test]
    fn test_is_same_tree_bfs_example_1() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert!(Solution::is_same_tree_bfs(p, q));
    }

    // Example 2:
    // Input: p = [1,2], q = [1,null,2]
    // Output: false
    #[test]
    fn test_is_same_tree_bfs_example_2() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2)]);
        let q = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert!(!Solution::is_same_tree_bfs(p, q));
    }

    // Example 3:
    // Input: p = [1,2,1], q = [1,1,2]
    // Output: false
    #[test]
    fn test_is_same_tree_bfs_example_3() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(1)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(1), Some(2)]);
        assert!(!Solution::is_same_tree_bfs(p, q));
    }

    // Example 1:
    // Input: p = [1,2,3], q = [1,2,3]
    // Output: true
    #[test]
    fn test_is_same_tree_iterative_dfs_example_1() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert!(Solution::is_same_tree_iterative_dfs(p, q));
    }

    // Example 2:
    // Input: p = [1,2], q = [1,null,2]
    // Output: false
    #[test]
    fn test_is_same_tree_iterative_dfs_example_2() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2)]);
        let q = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert!(!Solution::is_same_tree_iterative_dfs(p, q));
    }

    // Example 3:
    // Input: p = [1,2,1], q = [1,1,2]
    // Output: false
    #[test]
    fn test_is_same_tree_iterative_dfs_example_3() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(1)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(1), Some(2)]);
        assert!(!Solution::is_same_tree_iterative_dfs(p, q));
    }
}
