use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
pub struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .or_insert_with(Vec::new)
            .push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        match self.map.get(&key) {
            Some(item) => {
                let mut res: Option<&String> = None;
                let (mut left, mut right) = (0, item.len() as i32 - 1);

                while left <= right {
                    let mid = left + (right - left) / 2;

                    match item[mid as usize].0.cmp(&timestamp) {
                        Ordering::Less | Ordering::Equal => {
                            res = Some(&item[mid as usize].1);
                            left = mid + 1;
                        }
                        Ordering::Greater => right = mid - 1,
                    }
                }

                res.cloned().unwrap_or_default()
            }
            None => String::new(),
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    use crate::binary_search::q981_time_based_key_value_store::TimeMap;

    #[test]
    fn test_time_map() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
    }
}
