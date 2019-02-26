use crate::vector_based::VectorBasedDataStructure;

pub struct Queue<T> {
    entry: Vec<T>,
}

impl<T> Queue<T> {
    pub fn enqueue(&mut self, element: T) {
        self.entry.insert(0, element);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.entry.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.entry.len() == 0 {
            return None;
        }
        self.entry.get(self.entry.len() - 1)
    }
}

impl<T> From<Vec<T>> for Queue<T> {
    fn from(vec: Vec<T>) -> Self {
        Queue { entry: vec }
    }
}

impl<T> VectorBasedDataStructure<T> for Queue<T> {
    fn new() -> Self {
        Queue { entry: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Queue { entry: Vec::with_capacity(capacity) }
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
    use crate::queue::Queue;

    #[test]
    fn enqueue() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        assert_eq!(queue.len(), 5);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(queue.len(), 5);
        assert_eq!(queue.dequeue().unwrap(), 5);
        assert_eq!(queue.len(), 4);
        queue.dequeue();
        queue.dequeue();
        queue.dequeue();
        queue.dequeue();
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn peek() {
        let queue = Queue::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(queue.len(), 5);
        assert_eq!(queue.peek().unwrap(), &5);
        assert_eq!(queue.len(), 5);
        let empty_queue: Queue<i32> = Queue::new();
        assert_eq!(empty_queue.len(), 0);
        assert_eq!(empty_queue.peek(), None);
    }

    #[test]
    fn from() {
        let mut queue = Queue::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(queue.len(), 5);
        assert_eq!(queue.dequeue().unwrap(), 5);
        assert_eq!(queue.dequeue().unwrap(), 4);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.dequeue().unwrap(), 2);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn new() {
        let queue: Queue<i32> = Queue::new();
        assert_eq!(queue.len(), 0);
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(queue.capacity(), empty_vec.capacity());
    }

    #[test]
    fn with_capacity_and_capacity() {
        let mut queue: Queue<i32> = Queue::with_capacity(3);
        assert_eq!(queue.capacity(), 3);
        queue.enqueue(1);
        assert_eq!(queue.capacity(), 3);
        queue.enqueue(2);
        assert_eq!(queue.capacity(), 3);
        queue.enqueue(3);
        assert_eq!(queue.capacity(), 3);
        queue.enqueue(4);
        assert_eq!(queue.capacity(), 6);
    }

    #[test]
    fn len() {
        let mut queue = Queue::new();
        assert_eq!(queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.enqueue(3);
        assert_eq!(queue.len(), 3);
    }
}
