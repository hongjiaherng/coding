use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Node {
    key: i32,
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

pub struct LRUCache {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>, // dummy head, head.next is the actual head
    tail: Rc<RefCell<Node>>, // dummy tail, tail.prev is the actual tail
    capacity: usize,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key: key,
            val: value,
            next: None,
            prev: None,
        }))
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let dummy_head = Node::new(0, 0);
        let dummy_tail = Node::new(0, 0);

        dummy_head.borrow_mut().next = Some(dummy_tail.clone());
        dummy_tail.borrow_mut().prev = Some(dummy_head.clone());

        LRUCache {
            map: HashMap::new(),
            head: dummy_head,
            tail: dummy_tail,
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        // Get the ptr of the key from hashmap
        if let Some(node) = self.map.get(&key) {
            // With the ptr, remove it from current position, and reinsert back to the tail
            let val = node.borrow().val;
            self.remove_node(node.clone());
            self.add_to_tail(node.clone());
            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            // Update the node's value
            node.borrow_mut().val = value;

            // Remove existing node from the linked list
            self.remove_node(node.clone());

            // Insert the node back to tail
            self.add_to_tail(node.clone());
        } else {
            // Insert the new node to tail
            let new_node = Node::new(key, value);
            self.map.insert(key, new_node.clone());
            self.add_to_tail(new_node);

            // If capacity is full, remove head
            if self.map.len() > self.capacity {
                // Remove LRU from head
                let lru_node = self.head.borrow().next.as_ref().unwrap().clone();
                let lru_key = lru_node.borrow().key;

                self.remove_node(lru_node);
                self.map.remove(&lru_key);
            }
        }
    }

    fn add_to_tail(&mut self, node: Rc<RefCell<Node>>) {
        let prev = self.tail.borrow().prev.as_ref().unwrap().clone();

        // Link node <-> dummy_tail
        node.borrow_mut().next = Some(self.tail.clone());
        self.tail.borrow_mut().prev = Some(node.clone());

        // Link prev <-> node
        prev.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(prev);
    }

    fn remove_node(&self, node: Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().unwrap().clone();
        let next = node.borrow().next.as_ref().unwrap().clone();

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev.clone());
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use crate::linked_list::q146_lru_cache::LRUCache;

    // Input
    // ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
    // [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
    // Output
    // [null, null, null, 1, null, -1, null, -1, 3, 4]

    // Explanation
    // LRUCache lRUCache = new LRUCache(2);
    // lRUCache.put(1, 1); // cache is {1=1}
    // lRUCache.put(2, 2); // cache is {1=1, 2=2}
    // lRUCache.get(1);    // return 1
    // lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    // lRUCache.get(2);    // returns -1 (not found)
    // lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    // lRUCache.get(1);    // return -1 (not found)
    // lRUCache.get(3);    // return 3
    // lRUCache.get(4);    // return 4
    #[test]
    fn test_lru_cache() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1); // 1
        lru_cache.put(2, 2); // 1 -> 2
        assert_eq!(lru_cache.get(1), 1); // 2 -> 1
        lru_cache.put(3, 3); // remove 2 => 2 -> 1 => 1 -> 3
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(4, 4); // remove 1 => 1 -> 3 => 3 -> 4
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3); // 4 -> 3
        assert_eq!(lru_cache.get(4), 4); // 3 -> 4
    }
}
