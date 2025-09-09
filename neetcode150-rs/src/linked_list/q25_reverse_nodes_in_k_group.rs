use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k <= 1 {
            return head;
        }

        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut group_prev: *mut ListNode = &mut *dummy;

        loop {
            // Find kth node
            let kth: Option<*mut ListNode> = unsafe {
                (0..k).try_fold(group_prev, |curr, _| {
                    (*curr)
                        .next
                        .as_mut()
                        .map(|next| &mut **next as *mut ListNode)
                })
            };

            if let Some(kth_node) = kth {
                // Reverse the group between group_prev.next and kth node
                unsafe {
                    let mut prev = (*kth_node).next.take();
                    let mut curr = (*group_prev).next.take();
                    let tail = curr.as_deref_mut().unwrap() as *mut ListNode; // tail after reversing the group

                    for _ in 0..k {
                        let mut node = curr.take().unwrap();
                        curr = node.next.take();
                        node.next = prev;
                        prev = Some(node);
                    }

                    // Connect the reversed group to the previous group
                    (*group_prev).next = prev;
                    group_prev = tail;
                }
            } else {
                break;
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q25_reverse_nodes_in_k_group::Solution;
    use crate::linked_list::ListNode;

    // Input: head = [1,2,3,4,5], k = 2
    // Output: [2,1,4,3,5]
    #[test]
    fn test_reverse_k_group_2() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(vec![2, 1, 4, 3, 5]);
        assert_eq!(Solution::reverse_k_group(head, 2), expected);
    }

    // Input: head = [1,2,3,4,5], k = 3
    // Output: [3,2,1,4,5]
    #[test]
    fn test_reverse_k_group_3() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(vec![3, 2, 1, 4, 5]);
        assert_eq!(Solution::reverse_k_group(head, 3), expected);
    }
}
