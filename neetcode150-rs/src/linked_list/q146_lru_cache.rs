#![allow(unused)]
pub struct LRUCache {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        todo!()
    }

    pub fn get(&self, key: i32) -> i32 {
        todo!()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        todo!()
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
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);
        lru_cache.put(3, 3);
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }
}
