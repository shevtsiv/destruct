use std::cell::RefCell;
use std::rc::Rc;

struct QueueNode<T> {
    data: T,
    next: Option<Rc<RefCell<QueueNode<T>>>>,
}

pub struct Queue<T> {
    head: Option<Rc<RefCell<QueueNode<T>>>>,
    tail: Option<Rc<RefCell<QueueNode<T>>>>,
    len: usize,
}

impl<T> Queue<T> {
    pub fn enqueue(&mut self, element: T) {
        let new_tail = Rc::from(RefCell::new(QueueNode {
            data: element,
            next: None,
        }));
        if let Some(current_tail) = &mut self.tail {
            current_tail.borrow_mut().next.replace(new_tail.clone());
            self.tail.replace(new_tail);
        } else {
            self.head.replace(new_tail.clone());
            self.tail.replace(new_tail);
        }
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        return self.head.take().map(|head| {
            if Rc::ptr_eq(&head, self.tail.as_ref().unwrap()) {
                self.tail = None;
            }
            let head = Rc::try_unwrap(head)
                .unwrap_or_else(|_| panic!("Failed to unwrap Rc to the head of the Queue"));
            let inner_head = head.into_inner();
            self.head = inner_head.next.clone();
            self.len -= 1;
            return inner_head.data;
        });
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> From<Vec<T>> for Queue<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut queue = Queue::new();
        for element in vec {
            queue.enqueue(element);
        }
        queue
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Queue;

    #[test]
    fn enqueue() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.enqueue(3);
        assert_eq!(queue.len(), 3);
        queue.enqueue(4);
        assert_eq!(queue.len(), 4);
        queue.enqueue(5);
        assert_eq!(queue.len(), 5);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue().unwrap(), 2);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.dequeue().unwrap(), 4);
        assert_eq!(queue.dequeue().unwrap(), 5);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn enqueue_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue().unwrap(), 2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue().unwrap(), 3);
        queue.enqueue(4);
        assert_eq!(queue.dequeue().unwrap(), 4);
        assert_eq!(queue.dequeue(), None);
    }
}
