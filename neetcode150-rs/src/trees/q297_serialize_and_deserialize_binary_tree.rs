#![allow(unused)]
use crate::trees::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        todo!()
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!()
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
#[cfg(test)]
mod tests {
    use crate::trees::q297_serialize_and_deserialize_binary_tree::Codec;
    use crate::trees::TreeNode;

    // Example 1:
    // Input: root = [1,2,3,null,null,4,5]
    // Output: [1,2,3,null,null,4,5]
    #[test]
    fn test_serialize_and_deserialize_example_1() {
        let root = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
        ]);
        let codec = Codec::new();
        let serialized = codec.serialize(root.clone());
        let deserialized = codec.deserialize(serialized);
        assert_eq!(deserialized, root);
    }

    // Example 2:
    // Input: root = []
    // Output: []
    #[test]
    fn test_serialize_and_deserialize_example_2() {
        let root = TreeNode::from_vec(vec![]);
        let codec = Codec::new();
        let serialized = codec.serialize(root.clone());
        let deserialized = codec.deserialize(serialized);
        assert_eq!(deserialized, root);
    }
}
