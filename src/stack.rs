use crate::vector_based::VectorBasedDataStructure;

pub struct Stack<T> {
    entry: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, element: T) {
        self.entry.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        let entry_len = self.entry.len();
        if entry_len == 0 {
            None
        } else {
            Some(self.entry.remove(entry_len - 1))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let entry_len = self.entry.len();
        if entry_len == 0 {
            None
        } else {
            self.entry.get(entry_len - 1)
        }
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(vec: Vec<T>) -> Stack<T> {
        Stack { entry: vec }
    }
}

impl<T> VectorBasedDataStructure<T> for Stack<T> {
    fn new() -> Self {
        Stack { entry: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Stack { entry: Vec::with_capacity(capacity) }
    }

    fn len(&self) -> usize {
        self.entry.len()
    }

    fn capacity(&self) -> usize {
        self.entry.capacity()
    }
}

#[cfg(test)]
mod tests {
    use crate::vector_based::VectorBasedDataStructure;
    use crate::stack::Stack;

    #[test]
    fn push() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.len(), 0);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        assert_eq!(stack.len(), 5);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.pop().unwrap(), 5);
        assert_eq!(stack.len(), 4);
        stack.pop();
        stack.pop();
        stack.pop();
        stack.pop();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let stack = Stack::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.peek().unwrap(), &5);
        assert_eq!(stack.len(), 5);
        let empty_stack: Stack<i32> = Stack::new();
        assert_eq!(empty_stack.len(), 0);
        assert_eq!(empty_stack.peek(), None);
    }

    #[test]
    fn from() {
        let mut stack = Stack::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.pop().unwrap(), 5);
        assert_eq!(stack.pop().unwrap(), 4);
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn new() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.len(), 0);
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(stack.capacity(), empty_vec.capacity());
    }

    #[test]
    fn with_capacity_and_capacity() {
        let mut stack: Stack<i32> = Stack::with_capacity(3);
        assert_eq!(stack.capacity(), 3);
        stack.push(1);
        assert_eq!(stack.capacity(), 3);
        stack.push(2);
        assert_eq!(stack.capacity(), 3);
        stack.push(3);
        assert_eq!(stack.capacity(), 3);
        stack.push(4);
        assert_eq!(stack.capacity(), 6);
    }

    #[test]
    fn len() {
        let mut stack = Stack::new();
        assert_eq!(stack.len(), 0);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
    }
}
