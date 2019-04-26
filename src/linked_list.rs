use std::fmt::Debug;

#[derive(PartialOrd, PartialEq, Debug)]
pub struct LinkedListNode<T: PartialEq> {
    data: T,
    next_node: Option<Box<LinkedListNode<T>>>,
}

pub struct LinkedList<T: PartialEq> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T: PartialEq> LinkedListNode<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn get_next(&self) -> Option<&T> {
        if let Some(ref next) = self.next_node {
            return Some(next.get_data());
        } else {
            return None;
        }
    }
}

impl<T: PartialEq> LinkedList<T> {
    pub fn get_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn get_head_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn get_tail(&self) -> Option<&T> {
        // Empty LinkedList has no head
        if let Some(ref head) = self.head {
            let mut tail = head;
            // Loop until the last node is found
            while let Some(ref next) = tail.next_node {
                tail = next;
            }
            return Some(&tail.data);
        } else {
            return None;
        }
    }

    pub fn get_tail_mut(&mut self) -> Option<&mut T> {
        if let Some(ref mut head) = self.head {
            let mut tail = head;
            while tail.next_node.is_some() {
                tail = tail.next_node.as_mut().unwrap();
            }
            return Some(&mut tail.data);
        } else {
            return None;
        }
    }

    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn add(&mut self, value: T) {
        if let Some(head) = self.head.as_mut() {
            let mut tail = head;
            while let Some(ref mut next) = tail.next_node {
                tail = next;
            }
            tail.next_node = Some(Box::from(LinkedListNode {
                data: value,
                next_node: None,
            }));
        } else {
            self.head = Some(Box::from(LinkedListNode {
                data: value,
                next_node: None,
            }))
        }
    }

    pub fn add_first(&mut self, value: T) {
        self.head = Some(Box::from(LinkedListNode {
            data: value,
            next_node: self.head.take(),
        }));
    }

    pub fn add_after(&mut self, value: T, after: &T)
    where
        T: Debug,
    {
        let after_node = self
            .find_mut(after)
            .expect(format!("Cannot find LinkedListNode with value: {:?}", after).as_str());
        after_node.next_node = Some(Box::from(LinkedListNode {
            data: value,
            next_node: after_node.next_node.take(),
        }));
    }

    fn find_mut(&mut self, value: &T) -> Option<&mut LinkedListNode<T>> {
        if let Some(ref mut head) = self.head {
            let mut node = head;
            while &node.data != value {
                if let Some(ref mut next) = node.next_node {
                    node = next;
                } else {
                    return None;
                }
            }
            return Some(node);
        } else {
            return None;
        }
    }
    //
    fn find_match_mut<F>(&mut self, predicate: &F) -> Option<&mut LinkedListNode<T>>
    where
        F: Fn(&LinkedListNode<T>) -> bool,
    {
        if let Some(ref mut head) = self.head {
            let mut node = head;
            loop {
                if predicate(&node) {
                    return Some(node);
                }
                if let Some(ref mut next) = node.next_node {
                    node = next;
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }

    pub fn delete(&mut self, value: &T) {
        if self.find_mut(value).is_none() {
            return;
        }
        let prev = self.find_match_mut(&|node| {
            if let Some(ref next) = node.next_node {
                if &next.data == value {
                    return true;
                }
            }
            false
        });
        if let Some(prev) = prev {
            prev.next_node = prev.next_node.take().unwrap().next_node;
        } else {
            self.head = self.head.take().unwrap().next_node;
        }
    }

    pub fn delete_match<F>(&mut self, predicate: F)
    where
        F: Fn(&LinkedListNode<T>) -> bool,
    {
        if self.find_match_mut(&predicate).is_none() {
            return;
        }
        let prev = self.find_match_mut(&|node| {
            if let Some(ref next) = node.next_node {
                if predicate(next) {
                    return true;
                }
            }
            false
        });
        if let Some(prev) = prev {
            prev.next_node = prev.next_node.take().unwrap().next_node;
        } else {
            self.head = self.head.take().unwrap().next_node;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next_node;
            return head.data;
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    pub fn contains(&self, value: &T) -> bool {
        if let Some(mut node) = self.head.as_ref() {
            while &node.data != value {
                if let Some(ref next) = node.next_node {
                    node = next;
                } else {
                    return false;
                }
            }
            return true;
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
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
            head: Some(Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: None,
                })),
            })),
        };
        assert_eq!(has_tail.get_tail().unwrap(), &2);
        let head_only = LinkedList {
            head: Some(Box::from(LinkedListNode {
                data: 1,
                next_node: None,
            })),
        };
        assert_eq!(head_only.get_tail().unwrap(), &1);
        let deep_tail = LinkedList {
            head: Some(Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 4,
                            next_node: Some(Box::from(LinkedListNode {
                                data: 5,
                                next_node: None,
                            })),
                        })),
                    })),
                })),
            })),
        };
        assert_eq!(deep_tail.get_tail().unwrap(), &5);
    }

    #[test]
    fn add() {
        let mut list = LinkedList::new();
        list.add(1);
        assert_eq!(list.get_tail().unwrap(), &1);
        list.add(2);
        assert_eq!(list.get_tail().unwrap(), &2);
        list.add(3);
        assert_eq!(list.get_tail().unwrap(), &3);
        list.add(9);
        assert_eq!(list.get_tail().unwrap(), &9);
    }

    #[test]
    fn add_first() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        assert_eq!(list.get_head().unwrap(), &1);
        assert_eq!(list.get_tail().unwrap(), &2);
        list.add(3);
        assert_eq!(list.get_head().unwrap(), &1);
        assert_eq!(list.get_tail().unwrap(), &3);
        list.add_first(0);
        assert_eq!(list.get_head().unwrap(), &0);
        assert_eq!(list.get_tail().unwrap(), &3);
        list.add_first(15);
        assert_eq!(list.get_head().unwrap(), &15);
        assert_eq!(list.get_tail().unwrap(), &3);
    }

    #[test]
    fn add_after() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(4);
        assert_eq!(
            list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 4,
                            next_node: None,
                        })),
                    })),
                })),
            })
        );
        list.add_after(5, &3);
        assert_eq!(
            list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 5,
                            next_node: Some(Box::from(LinkedListNode {
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
    fn find_match_mut() {
        let mut deep_list = LinkedList {
            head: Some(Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 4,
                            next_node: Some(Box::from(LinkedListNode {
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
                .find_match_mut(
                    &|node| node.next_node.is_some() && node.next_node.as_ref().unwrap().data == 3
                )
                .unwrap(),
            &mut LinkedListNode {
                data: 2,
                next_node: Some(Box::from(LinkedListNode {
                    data: 3,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 4,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 5,
                            next_node: None,
                        })),
                    })),
                })),
            },
        );
        assert_eq!(
            deep_list.find_match_mut(&|node| node.data == 5).unwrap(),
            &mut LinkedListNode {
                data: 5,
                next_node: None,
            },
        );
    }

    #[test]
    fn delete() {
        let mut deep_list = LinkedList {
            head: Some(Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 4,
                            next_node: Some(Box::from(LinkedListNode {
                                data: 5,
                                next_node: None,
                            })),
                        })),
                    })),
                })),
            })),
        };
        deep_list.delete(&3);
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 4,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 5,
                            next_node: None,
                        })),
                    })),
                })),
            })
        );
        deep_list.delete(&1);
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 2,
                next_node: Some(Box::from(LinkedListNode {
                    data: 4,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 5,
                        next_node: None,
                    })),
                })),
            })
        );
        deep_list.delete(&5);
        deep_list.delete(&2);
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 4,
                next_node: None,
            })
        );
        deep_list.delete(&4);
        assert_eq!(deep_list.get_head(), None);
    }

    #[test]
    fn delete_match() {
        let mut deep_list = LinkedList {
            head: Some(Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 4,
                            next_node: Some(Box::from(LinkedListNode {
                                data: 5,
                                next_node: None,
                            })),
                        })),
                    })),
                })),
            })),
        };
        deep_list.delete_match(|node| node.get_data() == &3);
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 4,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 5,
                            next_node: None,
                        })),
                    })),
                })),
            })
        );
        deep_list.delete_match(|node| {
            if let Some(next) = node.get_next() {
                if next == &2 {
                    return true;
                }
            }
            return false;
        });
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 2,
                next_node: Some(Box::from(LinkedListNode {
                    data: 4,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 5,
                        next_node: None,
                    })),
                })),
            })
        );
        deep_list.delete_match(|node| node.get_next().is_none());
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 2,
                next_node: Some(Box::from(LinkedListNode {
                    data: 4,
                    next_node: None,
                })),
            })
        );
        deep_list.delete_match(|node| node.get_data() == &2);
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 4,
                next_node: None,
            })
        );
        deep_list.delete_match(|node| node.get_data() / 2 == 2);
        assert_eq!(deep_list.get_head(), None);
    }

    #[test]
    fn pop() {
        let mut list = LinkedList {
            head: Some(Box::from(LinkedListNode {
                data: 1,
                next_node: Some(Box::from(LinkedListNode {
                    data: 2,
                    next_node: Some(Box::from(LinkedListNode {
                        data: 3,
                        next_node: Some(Box::from(LinkedListNode {
                            data: 4,
                            next_node: Some(Box::from(LinkedListNode {
                                data: 5,
                                next_node: None,
                            })),
                        })),
                    })),
                })),
            })),
        };
        assert_eq!(list.pop().unwrap(), 1);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.pop().unwrap(), 5);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        list.add(1);
        assert_eq!(list.peek(), Some(&1));
        list.add_first(2);
        assert_eq!(list.peek(), Some(&2));
        list.add_first(3);
        assert_eq!(list.peek(), Some(&3));
        list.add(4);
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn contains() {
        let mut list = LinkedList::new();
        assert!(!list.contains(&0));
        list.add(0);
        assert!(list.contains(&0));
        list.add(1);
        list.add(5);
        list.add(9);
        assert!(list.contains(&1));
        assert!(list.contains(&5));
        assert!(list.contains(&9));
        assert!(!list.contains(&10));
        list.add(10);
        assert!(list.contains(&10));
    }
}
