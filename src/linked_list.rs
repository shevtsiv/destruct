use std::rc::Rc;

#[derive(PartialOrd, PartialEq, Debug)]
pub struct LinkedListNode<T: PartialEq> {
    data: T,
    next_node: Option<Rc<LinkedListNode<T>>>,
}

pub struct LinkedList<T: PartialEq> {
    head: Option<Rc<LinkedListNode<T>>>,
}

impl<T: PartialEq> LinkedListNode<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
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

impl<T: PartialEq> LinkedList<T> {
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

    // Is this return value correct?
    pub fn get_tail_mut(&mut self) -> Option<&mut LinkedListNode<T>> {
        // Empty LinkedList has no head
        if self.head.is_some() {
            // If head is a tail (LinkedList with only element)
            if self.head.as_ref().unwrap().next_node.is_none() {
                return Some(Rc::get_mut(self.head.as_mut().unwrap()).unwrap());
            }
            let mut tail = Rc::get_mut(self.head.as_mut().unwrap()).unwrap();
            loop { // Loop until the last node is found
                if tail.next_node.is_none() {
                    // It is the last node in the list
                    return Some(tail);
                } else {
                    // Move on to the next node
                    tail = Rc::get_mut(tail.next_node.as_mut().unwrap()).unwrap();
                }
            }
        } else {
            return None;
        }
    }

    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn add(&mut self, value: T) {
        let tail = self.get_tail_mut();
        match tail {
            Some(tail) => {
                tail.set_next(LinkedListNode { data: value, next_node: None });
            }
            None => {
                self.head = Some(Rc::from(LinkedListNode { data: value, next_node: None }))
            }
        }
    }

    pub fn add_first(&mut self, value: T) {
        let old_head = self.head.as_ref().unwrap().to_owned();
        let new_head = LinkedListNode { data: value, next_node: Some(old_head) };
        self.head = Some(Rc::from(new_head));
    }

    pub fn add_after(&mut self, value: T, after: &T) {
        let after_node = self.find_mut(after).unwrap();
        let next_node = after_node.get_next().to_owned();
        after_node.set_next(LinkedListNode { data: value, next_node });
    }

    pub fn find(&self, value: &T) -> Option<&Rc<LinkedListNode<T>>> {
        if let Some(head) = &self.head {
            if &head.data == value {
                return Some(head);
            }
            let mut tail = head.next_node.as_ref();
            if tail.is_none() {
                return None;
            }
            loop {
                if tail.is_none() {
                    return None;
                } else {
                    if &tail.unwrap().data == value {
                        return tail;
                    }
                    tail = tail.unwrap().next_node.as_ref();
                }
            }
        } else {
            return None;
        }
    }

    pub fn find_mut(&mut self, value: &T) -> Option<&mut LinkedListNode<T>> {
        if self.head.is_some() {
            if &self.head.as_ref().unwrap().data == value {
                return Some(Rc::get_mut(self.head.as_mut().unwrap()).unwrap());
            }
            let mut tail = Rc::get_mut(self.head.as_mut().unwrap()).unwrap();
            loop {
                if &tail.data == value {
                    return Some(tail);
                } else if tail.next_node.is_none() {
                    return None;
                } else {
                    tail = Rc::get_mut(tail.next_node.as_mut().unwrap()).unwrap();
                }
            }
        } else {
            return None;
        }
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
                data: 1,
                next_node: Some(Rc::from(LinkedListNode {
                    data: 2,
                    next_node: None,
                })),
            }))
        };
        assert_eq!(has_tail.get_tail().unwrap().get_data(), &2);
        let head_only = LinkedList {
            head: Some(Rc::from(LinkedListNode { data: 1, next_node: None }))
        };
        assert_eq!(head_only.get_tail().unwrap().get_data(), &1);
        let deep_tail = LinkedList {
            head: Some(Rc::from(LinkedListNode {
                data: 1,
                next_node: Some(Rc::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Rc::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Rc::from(LinkedListNode {
                            data: 4,
                            next_node: Some(Rc::from(LinkedListNode {
                                data: 5,
                                next_node: None,
                            })),
                        })),
                    })),
                })),
            }))
        };
        assert_eq!(deep_tail.get_tail().unwrap().get_data(), &5);
    }

    #[test]
    fn add() {
        let mut list = LinkedList::new();
        list.add(1);
        assert_eq!(list.get_tail().unwrap().get_data(), &1);
        list.add(2);
        assert_eq!(list.get_tail().unwrap().get_data(), &2);
        list.add(3);
        assert_eq!(list.get_tail().unwrap().get_data(), &3);
        list.add(9);
        assert_eq!(list.get_tail().unwrap().get_data(), &9);
    }

    #[test]
    fn add_first() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        assert_eq!(list.get_head().unwrap().get_data(), &1);
        assert_eq!(list.get_tail().unwrap().get_data(), &2);
        list.add(3);
        assert_eq!(list.get_head().unwrap().get_data(), &1);
        assert_eq!(list.get_tail().unwrap().get_data(), &3);
        list.add_first(0);
        assert_eq!(list.get_head().unwrap().get_data(), &0);
        assert_eq!(list.get_tail().unwrap().get_data(), &3);
        list.add_first(15);
        assert_eq!(list.get_head().unwrap().get_data(), &15);
        assert_eq!(list.get_tail().unwrap().get_data(), &3);
    }

    #[test]
    fn add_after() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(4);
        assert_eq!(list.find(&1).unwrap().to_owned(), Rc::from(LinkedListNode {
            data: 1,
            next_node: Some(Rc::from(LinkedListNode {
                data: 2,
                next_node: Some(Rc::from(LinkedListNode {
                    data: 3,
                    next_node: Some(Rc::from(LinkedListNode {
                        data: 4,
                        next_node: None,
                    })),
                })),
            })),
        }));
        list.add_after(5, &3);
        assert_eq!(list.find(&1).unwrap().to_owned(), Rc::from(LinkedListNode {
            data: 1,
            next_node: Some(Rc::from(LinkedListNode {
                data: 2,
                next_node: Some(Rc::from(LinkedListNode {
                    data: 3,
                    next_node: Some(Rc::from(LinkedListNode {
                        data: 5,
                        next_node: Some(Rc::from(LinkedListNode {
                            data: 4,
                            next_node: None,
                        })),
                    })),
                })),
            })),
        }));
    }

    #[test]
    fn find() {
        let deep_tail = LinkedList {
            head: Some(Rc::from(LinkedListNode {
                data: 1,
                next_node: Some(Rc::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Rc::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Rc::from(LinkedListNode {
                            data: 4,
                            next_node: Some(Rc::from(LinkedListNode {
                                data: 5,
                                next_node: None,
                            })),
                        })),
                    })),
                })),
            }))
        };
        assert_eq!(deep_tail.find(&1).unwrap(), deep_tail.get_head().unwrap());
        assert_eq!(deep_tail.find(&2).unwrap().get_next().as_ref().unwrap().get_data(), &3);
        assert_eq!(deep_tail.find(&3).unwrap().get_next().as_ref().unwrap().get_data(), &4);
        assert_eq!(deep_tail.find(&4).unwrap().get_next().as_ref().unwrap().get_data(), &5);
        assert_eq!(deep_tail.find(&5).unwrap().get_next(), &None);
        assert_eq!(deep_tail.find(&6), None);
    }
}
