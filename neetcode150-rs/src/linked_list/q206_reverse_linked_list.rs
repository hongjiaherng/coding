use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None; // left pointer
        let mut curr = head; // right pointer

        while let Some(mut node) = curr {
            curr = node.next.take(); // Update right pointer
            node.next = prev;
            prev = Some(node); // Update left pointer
        }
        prev
    }

    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn recurse(
            prev: Option<Box<ListNode>>,
            curr: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match curr {
                None => prev,
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = prev;
                    recurse(Some(node), next)
                }
            }
        }
        recurse(None, head)
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::ListNode;

    use crate::linked_list::q206_reverse_linked_list::Solution;

    // Input: head = [1,2,3,4,5]
    // Output: [5,4,3,2,1]
    #[test]
    fn test_reverse_list_example_1() {
        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![5, 4, 3, 2, 1];
        let result = Solution::reverse_list(ListNode::from_vec(input));
        assert_eq!(result, ListNode::from_vec(expected));
    }

    // Input: head = [1,2]
    // Output: [2,1]
    #[test]
    fn test_reverse_list_example_2() {
        let input = vec![1, 2];
        let expected = vec![2, 1];
        let result = Solution::reverse_list(ListNode::from_vec(input));
        assert_eq!(result, ListNode::from_vec(expected));
    }

    // Input: head = []
    // Output: []
    #[test]
    fn test_reverse_list_example_3() {
        let input = vec![];
        let expected = vec![];
        let result = Solution::reverse_list(ListNode::from_vec(input));
        assert_eq!(result, ListNode::from_vec(expected));
    }

    // Input: head = [1,2,3,4,5]
    // Output: [5,4,3,2,1]
    #[test]
    fn test_reverse_list_recursive_example_1() {
        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![5, 4, 3, 2, 1];
        let result = Solution::reverse_list_recursive(ListNode::from_vec(input));
        assert_eq!(result, ListNode::from_vec(expected));
    }

    // Input: head = [1,2]
    // Output: [2,1]
    #[test]
    fn test_reverse_list_recursive_example_2() {
        let input = vec![1, 2];
        let expected = vec![2, 1];
        let result = Solution::reverse_list_recursive(ListNode::from_vec(input));
        assert_eq!(result, ListNode::from_vec(expected));
    }

    // Input: head = []
    // Output: []
    #[test]
    fn test_reverse_list_recursive_example_3() {
        let input = vec![];
        let expected = vec![];
        let result = Solution::reverse_list_recursive(ListNode::from_vec(input));
        assert_eq!(result, ListNode::from_vec(expected));
    }
}
