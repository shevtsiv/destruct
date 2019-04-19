use std::fmt::Debug;
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

    pub fn get_next(&self) -> Option<&Rc<LinkedListNode<T>>> {
        self.next_node.as_ref()
    }

    pub fn get_next_mut(&mut self) -> Option<&mut Rc<LinkedListNode<T>>> {
        self.next_node.as_mut()
    }

    pub fn set_next(&mut self, next: Option<Rc<LinkedListNode<T>>>) {
        self.next_node = next;
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
        if let Some(mut head) = self.head.as_ref() {
            // Loop until the last node is found
            while let Some(next) = head.next_node.as_ref() {
                head = next;
            }
            return Some(head);
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
            loop {
                // Loop until the last node is found
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
                tail.set_next(Some(Rc::from(LinkedListNode {
                    data: value,
                    next_node: None,
                })));
            }
            None => {
                self.head = Some(Rc::from(LinkedListNode {
                    data: value,
                    next_node: None,
                }))
            }
        }
    }

    pub fn add_first(&mut self, value: T) {
        if let Some(head) = self.head.as_ref() {
            let old_head = head.to_owned();
            let new_head = LinkedListNode {
                data: value,
                next_node: Some(old_head),
            };
            self.head = Some(Rc::from(new_head));
        } else {
            let new_head = LinkedListNode {
                data: value,
                next_node: None,
            };
            self.head = Some(Rc::from(new_head));
        }
    }

    pub fn add_after(&mut self, value: T, after: &T)
    where
        T: Debug,
    {
        let after_node = self
            .find_mut(after)
            .expect(format!("Cannot find LinkedListNode with value: {:?}", after).as_str());
        if let Some(next) = after_node.get_next() {
            after_node.set_next(Some(Rc::from(LinkedListNode {
                data: value,
                next_node: Some(next.to_owned()),
            })));
        } else {
            after_node.set_next(Some(Rc::from(LinkedListNode {
                data: value,
                next_node: None,
            })));
        }
    }

    pub fn find(&self, value: &T) -> Option<&Rc<LinkedListNode<T>>> {
        if let Some(mut node) = self.head.as_ref() {
            loop {
                if &node.data == value {
                    return Some(node);
                }
                if let Some(next) = &node.next_node {
                    node = next;
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }

    pub fn find_mut(&mut self, value: &T) -> Option<&mut LinkedListNode<T>> {
        if let Some(head) = self.head.as_mut() {
            let mut node = Rc::get_mut(head).unwrap();
            loop {
                if &node.data == value {
                    return Some(node);
                }
                if let Some(next) = node.next_node.as_mut() {
                    node = Rc::get_mut(next).unwrap();
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }

    pub fn find_match_mut<F>(&mut self, predicate: F) -> Option<&mut LinkedListNode<T>>
    where
        F: Fn(&LinkedListNode<T>) -> bool,
    {
        if let Some(head) = self.head.as_mut() {
            let mut node = Rc::get_mut(head).unwrap();
            loop {
                if predicate(&node) {
                    return Some(node);
                }
                if let Some(next) = node.next_node.as_mut() {
                    node = Rc::get_mut(next).unwrap();
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, LinkedListNode};
    use std::rc::Rc;

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
            })),
        };
        assert_eq!(has_tail.get_tail().unwrap().get_data(), &2);
        let head_only = LinkedList {
            head: Some(Rc::from(LinkedListNode {
                data: 1,
                next_node: None,
            })),
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
            })),
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
        assert_eq!(
            list.find(&1).unwrap().to_owned(),
            Rc::from(LinkedListNode {
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
            })
        );
        list.add_after(5, &3);
        assert_eq!(
            list.find(&1).unwrap().to_owned(),
            Rc::from(LinkedListNode {
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
            })
        );
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
            })),
        };
        assert_eq!(deep_tail.find(&1).unwrap(), deep_tail.get_head().unwrap());
        assert_eq!(
            deep_tail.find(&2).unwrap().get_next().unwrap().get_data(),
            &3
        );
        assert_eq!(
            deep_tail.find(&3).unwrap().get_next().unwrap().get_data(),
            &4
        );
        assert_eq!(
            deep_tail.find(&4).unwrap().get_next().unwrap().get_data(),
            &5
        );
        assert_eq!(deep_tail.find(&5).unwrap().get_next(), None);
        assert_eq!(deep_tail.find(&6), None);
    }

    #[test]
    fn find_match_mut() {
        let mut deep_list = LinkedList {
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
            })),
        };
        assert_eq!(
            deep_list
                .find_match_mut(|node| node.has_next() && node.get_next().unwrap().get_data() == &3)
                .unwrap(),
            &mut LinkedListNode {
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
            },
        );
        assert_eq!(
            deep_list
                .find_match_mut(|node| node.get_data() == &5)
                .unwrap(),
            &mut LinkedListNode {
                data: 5,
                next_node: None,
            },
        );
    }
}
