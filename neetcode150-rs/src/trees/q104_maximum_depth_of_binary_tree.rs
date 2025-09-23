use crate::trees::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Iterative bfs, deque
        let mut depth = 0;
        let mut deque = VecDeque::new();

        if let Some(root_rc) = root {
            deque.push_back(root_rc);
        }

        while !deque.is_empty() {
            // nodes at the current level
            let nodes: Vec<_> = deque.drain(..deque.len()).collect();
            for node_rc in nodes {
                let node = node_rc.borrow();
                if let Some(left_rc) = &node.left {
                    deque.push_back(left_rc.clone());
                }
                if let Some(right_rc) = &node.right {
                    deque.push_back(right_rc.clone());
                }
            }
            depth += 1;
        }

        depth
    }

    pub fn recursive_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node_rc) = root {
            1 + Self::recursive_dfs(node_rc.borrow().left.clone())
                .max(Self::recursive_dfs(node_rc.borrow().right.clone()))
        } else {
            0
        }
    }

    pub fn iterative_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // iterative dfs, stack
        if root.is_none() {
            return 0;
        }

        let mut max_depth = 0;
        let mut stack = vec![(root.unwrap(), 1)];

        while let Some((node_rc, depth)) = stack.pop() {
            max_depth = max_depth.max(depth);

            let node = node_rc.borrow();
            if let Some(left_rc) = &node.left {
                stack.push((left_rc.clone(), depth + 1));
            }

            if let Some(right_rc) = &node.right {
                stack.push((right_rc.clone(), depth + 1));
            }
        }

        max_depth
    }
}

#[cfg(test)]
mod tests {
    use crate::trees::q104_maximum_depth_of_binary_tree::Solution;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [3,9,20,null,null,15,7]
    // Output: 3
    #[test]
    fn test_max_depth_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }

    // Example 2:
    // Input: root = [1,null,2]
    // Output: 2
    #[test]
    fn test_max_depth_example_2() {
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(root), 2);
    }

    // Example 3:
    // Input: root = []
    // Output: 0
    #[test]
    fn test_max_depth_example_3() {
        let root = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::max_depth(root), 0);
    }

    #[test]
    fn test_recursive_dfs_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::recursive_dfs(root), 3);
    }

    #[test]
    fn test_recursive_dfs_example_2() {
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::recursive_dfs(root), 2);
    }

    #[test]
    fn test_recursive_dfs_example_3() {
        let root = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::recursive_dfs(root), 0);
    }

    #[test]
    fn test_iterative_dfs_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::iterative_dfs(root), 3);
    }

    #[test]
    fn test_iterative_dfs_example_2() {
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::iterative_dfs(root), 2);
    }

    #[test]
    fn test_iterative_dfs_example_3() {
        let root = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::iterative_dfs(root), 0);
    }
}
