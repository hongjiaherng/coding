use std::i64;

pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

pub struct MinStack2 {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

pub struct CleverMinStack {
    stack: Vec<i32>,
    min: i32,
}

pub struct NoOverflowCleverMinStack {
    stack: Vec<i64>,
    min: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || &val < self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        } else {
            self.min_stack.push(*self.min_stack.last().unwrap());
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        *self
            .stack
            .last()
            .expect("Stack is empty, top() is not allowed")
    }

    pub fn get_min(&self) -> i32 {
        *self
            .min_stack
            .last()
            .expect("Stack is empty, get_min() is not allowed")
    }
}

impl MinStack2 {
    pub fn new() -> Self {
        MinStack2 {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.last().map_or(true, |&min| val <= min) {
            self.min_stack.push(val);
        }
    }

    pub fn pop(&mut self) {
        if let Some(popped) = self.stack.pop() {
            if Some(&popped) <= self.min_stack.last() {
                self.min_stack.pop();
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self
            .stack
            .last()
            .expect("Stack is empty, top() is not allowed")
    }

    pub fn get_min(&self) -> i32 {
        *self
            .min_stack
            .last()
            .expect("Stack is empty, get_min() is not allowed")
    }
}

impl CleverMinStack {
    pub fn new() -> Self {
        CleverMinStack {
            stack: Vec::new(),
            min: i32::MAX,
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.min = val;
            self.stack.push(0); // encoded = actual - min = min - min = 0
        } else {
            let encoded = val - self.min; // encoded = new_min - min
            self.stack.push(encoded);
            if val < self.min {
                self.min = val;
            }
        }
    }

    pub fn pop(&mut self) {
        if let Some(encoded) = self.stack.pop() {
            if encoded <= 0 {
                self.min = self.min - encoded; // prev_min = cur_min - encoded
            }
        }
    }

    pub fn top(&self) -> i32 {
        if let Some(&encoded) = self.stack.last() {
            if encoded <= 0 {
                self.min
            } else {
                encoded + self.min
            }
        } else {
            panic!("Stack is empty, top() is not allowed");
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
}

impl NoOverflowCleverMinStack {
    pub fn new() -> Self {
        NoOverflowCleverMinStack {
            stack: Vec::new(),
            min: i64::MAX,
        }
    }

    pub fn push(&mut self, val: i32) {
        let big_val = val as i64;
        if self.stack.is_empty() {
            self.min = big_val;
            self.stack.push(0);
        } else {
            let encoded = big_val - self.min; // encoded = new_min - min
            self.stack.push(encoded);
            if big_val < self.min {
                self.min = big_val;
            }
        }
    }

    pub fn pop(&mut self) {
        if let Some(encoded) = self.stack.pop() {
            if encoded <= 0 {
                self.min = self.min - encoded; // prev_min = cur_min - encoded
            }
        }
    }

    pub fn top(&self) -> i32 {
        if let Some(&encoded) = self.stack.last() {
            if encoded <= 0 {
                self.min as i32
            } else {
                (encoded + self.min) as i32
            }
        } else {
            panic!("Stack is empty, top() is not allowed");
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min as i32
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use crate::stack::q155_min_stack::CleverMinStack;
    use crate::stack::q155_min_stack::MinStack;
    use crate::stack::q155_min_stack::MinStack2;
    use crate::stack::q155_min_stack::NoOverflowCleverMinStack;

    /// Test case 1
    /// ```
    /// Input
    /// ["MinStack","push","push","push","getMin","pop","top","getMin"]
    /// [[],[-2],[0],[-3],[],[],[],[]]
    /// Output
    /// [null,null,null,null,-3,null,0,-2]
    ///  ```
    #[test]
    fn test1_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }

    /// Test case 2
    /// ```
    /// ["MinStack","push","push","push","getMin","pop","getMin"]
    /// [[],[0],[1],[0],[],[],[]]
    /// ```
    #[test]
    fn test2_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(0);
        min_stack.push(1);
        min_stack.push(0);
        assert_eq!(min_stack.get_min(), 0);
        min_stack.pop();
        assert_eq!(min_stack.get_min(), 0);
    }

    #[test]
    fn test1_min_stack2() {
        let mut min_stack = MinStack2::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }

    /// Test case 2
    /// ```
    /// ["MinStack","push","push","push","getMin","pop","getMin"]
    /// [[],[0],[1],[0],[],[],[]]
    /// ```
    #[test]
    fn test2_min_stack2() {
        let mut min_stack = MinStack2::new();
        min_stack.push(0);
        min_stack.push(1);
        min_stack.push(0);
        assert_eq!(min_stack.get_min(), 0);
        min_stack.pop();
        assert_eq!(min_stack.get_min(), 0);
    }

    /// Test case 1
    /// ```
    /// Input
    /// ["MinStack","push","push","push","getMin","pop","top","getMin"]
    /// [[],[-2],[0],[-3],[],[],[],[]]
    /// Output
    /// [null,null,null,null,-3,null,0,-2]
    ///  ```
    #[test]
    fn test1_clever_min_stack() {
        let mut min_stack = CleverMinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }

    /// Test case 2
    /// ```
    /// ["MinStack","push","push","push","getMin","pop","getMin"]
    /// [[],[0],[1],[0],[],[],[]]
    /// ```
    #[test]
    fn test2_clever_min_stack() {
        let mut min_stack = CleverMinStack::new();
        min_stack.push(0);
        min_stack.push(1);
        min_stack.push(0);
        assert_eq!(min_stack.get_min(), 0);
        min_stack.pop();
        assert_eq!(min_stack.get_min(), 0);
    }

    #[test]
    fn test1_no_overflow_clever_min_stack() {
        let mut min_stack = NoOverflowCleverMinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }

    /// Test case 2
    /// ```
    /// ["MinStack","push","push","push","getMin","pop","getMin"]
    /// [[],[0],[1],[0],[],[],[]]
    /// ```
    #[test]
    fn test2_no_overflow_clever_min_stack() {
        let mut min_stack = NoOverflowCleverMinStack::new();
        min_stack.push(0);
        min_stack.push(1);
        min_stack.push(0);
        assert_eq!(min_stack.get_min(), 0);
        min_stack.pop();
        assert_eq!(min_stack.get_min(), 0);
    }
}
