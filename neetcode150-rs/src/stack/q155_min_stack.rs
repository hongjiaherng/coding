struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || val < *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        } else {
            self.min_stack.push(*self.min_stack.last().unwrap());
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().expect("Stack is empty")
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().expect("Stack is empty")
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
    use crate::stack::q155_min_stack::MinStack;

    /// Test case 1
    /// ```
    /// Input
    /// ["MinStack","push","push","push","getMin","pop","top","getMin"]
    /// [[],[-2],[0],[-3],[],[],[],[]]
    /// Output
    /// [null,null,null,null,-3,null,0,-2]
    ///  ```
    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        println!("stack: {:?}, min_stack: {:?}", min_stack.stack, min_stack.min_stack);
        min_stack.push(0);
        println!("stack: {:?}, min_stack: {:?}", min_stack.stack, min_stack.min_stack);
        min_stack.push(-3);
        println!("stack: {:?}, min_stack: {:?}", min_stack.stack, min_stack.min_stack);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        println!("stack: {:?}, min_stack: {:?}", min_stack.stack, min_stack.min_stack);
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
