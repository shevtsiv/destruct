use std::rc::Rc;

#[derive(PartialOrd, PartialEq, Debug)]
pub struct LinkedListNode<T> {
    data: Option<T>,
    next_node: Option<Rc<LinkedListNode<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Rc<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T> {
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn get_data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    pub fn get_next(&self) -> &Option<Rc<LinkedListNode<T>>> {
        &self.next_node
    }

    pub fn get_next_mut(&mut self) -> Option<&mut Rc<LinkedListNode<T>>> {
        self.next_node.as_mut()
    }

    pub fn set_next(&mut self, next: LinkedListNode<T>) {
        self.next_node = Some(Rc::from(next));
    }

    pub fn has_next(&self) -> bool {
        self.next_node.is_some()
    }
}

impl<T> LinkedList<T> {
    pub fn get_head(&self) -> Option<&Rc<LinkedListNode<T>>> {
        self.head.as_ref()
    }

    pub fn get_head_mut(&mut self) -> Option<&mut Rc<LinkedListNode<T>>> {
        self.head.as_mut()
    }

    pub fn get_tail(&self) -> Option<&Rc<LinkedListNode<T>>> {
        // Empty LinkedList has no head
        if let Some(head) = &self.head {
            // Starting from the second node
            let mut tail = head.next_node.as_ref();
            // If head is a tail (LinkedList with only element)
            if tail.is_none() {
                return Some(head);
            }
            loop { // Loop until the last node is found
                if tail.unwrap().next_node.is_none() {
                    // It is the last node in the list
                    return tail;
                } else {
                    // Move on to the next node
                    tail = tail.unwrap().next_node.as_ref();
                }
            }
        } else {
            return None;
        }
    }

    pub fn new() -> Self {
        LinkedList { head: None }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::linked_list::{LinkedList, LinkedListNode};

    #[test]
    fn new() {
        let _linked_list: LinkedList<i32> = LinkedList::new();
    }

    #[test]
    fn get_tail() {
        let empty: LinkedList<i32> = LinkedList::new();
        assert_eq!(empty.get_tail(), None);
        let has_tail = LinkedList {
            head: Some(Rc::from(LinkedListNode {
                data: Some(1),
                next_node: Some(Rc::from(LinkedListNode {
                    data: Some(2),
                    next_node: None,
                })),
            }))
        };
        assert_eq!(has_tail.get_tail().unwrap().get_data().unwrap(), &2);
        let head_only = LinkedList {
            head: Some(Rc::from(LinkedListNode { data: Some(1), next_node: None }))
        };
        assert_eq!(head_only.get_tail().unwrap().get_data().unwrap(), &1);
        let deep_tail = LinkedList {
            head: Some(Rc::from(LinkedListNode {
                data: Some(1),
                next_node: Some(Rc::from(LinkedListNode {
                    data: Some(2),
                    next_node: Some(Rc::from(LinkedListNode {
                        data: Some(3),
                        next_node: Some(Rc::from(LinkedListNode {
                            data: Some(4),
                            next_node: Some(Rc::from(LinkedListNode {
                                data: Some(5),
                                next_node: None,
                            })),
                        })),
                    })),
                })),
            }))
        };
        assert_eq!(deep_tail.get_tail().unwrap().get_data().unwrap(), &5);
    }
}
