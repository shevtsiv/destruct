pub struct LinkedStack<T> {
    size: usize,
    head: Option<Box<LinkedStackNode<T>>>,
}

struct LinkedStackNode<T> {
    data: T,
    next: Option<Box<LinkedStackNode<T>>>,
}

impl<T> LinkedStack<T> {
    pub fn push(&mut self, new_value: T) {
        let old_head = self.head.take();
        self.head.replace(Box::from(LinkedStackNode {
            data: new_value,
            next: old_head,
        }));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            self.size -= 1;
            head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    pub fn new() -> Self {
        LinkedStack {
            size: 0,
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T> From<Vec<T>> for LinkedStack<T> {
    fn from(vec: Vec<T>) -> LinkedStack<T> {
        vec.into_iter().fold(LinkedStack::new(), |mut stack, elem| {
            stack.push(elem);
            stack
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_stack::LinkedStack;

    #[test]
    fn push() {
        let mut stack: LinkedStack<i32> = LinkedStack::new();
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
        let mut stack = LinkedStack::from(vec![1, 2, 3, 4, 5]);
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
        let stack = LinkedStack::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.peek().unwrap(), &5);
        assert_eq!(stack.len(), 5);
        let empty_stack: LinkedStack<i32> = LinkedStack::new();
        assert_eq!(empty_stack.len(), 0);
        assert_eq!(empty_stack.peek(), None);
    }

    #[test]
    fn from() {
        let mut stack = LinkedStack::from(vec![1, 2, 3, 4, 5]);
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
        let stack: LinkedStack<i32> = LinkedStack::new();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn len() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.len(), 0);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
    }
}
