use std::fmt::Debug;

#[derive(PartialOrd, PartialEq, Debug)]
struct LinkedListNode<T: PartialEq> {
    data: T,
    next_node: Option<Box<LinkedListNode<T>>>,
}

#[derive(PartialEq, Debug)]
pub struct LinkedList<T: PartialEq> {
    head: Option<Box<LinkedListNode<T>>>,
    len: usize,
}

impl<T: PartialEq> LinkedListNode<T> {
    pub fn has_next(&self) -> bool {
        self.next_node.is_some()
    }
}

impl<T: PartialEq> LinkedList<T> {
    pub fn get_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn set_head(&mut self, value: T) {
        if let Some(ref mut head) = self.head {
            head.data = value;
        } else {
            self.head = Some(Box::from(LinkedListNode {
                data: value,
                next_node: None,
            }));
            self.len += 1;
        }
    }

    pub fn get_tail(&self) -> Option<&T> {
        // Empty LinkedList has no head
        return if let Some(ref head) = self.head {
            let mut tail = head;
            // Loop until the last node is found
            while let Some(ref next) = tail.next_node {
                tail = next;
            }
            Some(&tail.data)
        } else {
            None
        };
    }

    pub fn get_tail_mut(&mut self) -> Option<&mut T> {
        return if let Some(ref mut head) = self.head {
            let mut tail = head;
            while tail.has_next() {
                tail = tail.next_node.as_mut().unwrap();
            }
            Some(&mut tail.data)
        } else {
            None
        };
    }

    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
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
            }));
        }
        self.len += 1;
    }

    pub fn add_first(&mut self, value: T) {
        self.head = Some(Box::from(LinkedListNode {
            data: value,
            next_node: self.head.take(),
        }));
        self.len += 1;
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
        self.len += 1;
    }

    fn find_mut(&mut self, value: &T) -> Option<&mut LinkedListNode<T>> {
        return if let Some(ref mut head) = self.head {
            let mut node = head;
            while &node.data != value {
                if let Some(ref mut next) = node.next_node {
                    node = next;
                } else {
                    return None;
                }
            }
            Some(node)
        } else {
            None
        };
    }

    pub fn delete(&mut self, value: &T) {
        if !self.contains(value) {
            return;
        }
        let prev = self.get_prev_node(value);
        if let Some(prev) = prev {
            prev.next_node = prev.next_node.take().unwrap().next_node;
        } else {
            self.head = self.head.take().unwrap().next_node;
        }
        self.len -= 1;
    }

    pub fn delete_match<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        if let Some(prev) = self.get_prev_node_match(&predicate) {
            if let Some(node_to_delete) = prev.next_node.take() {
                prev.next_node = node_to_delete.next_node;
                self.len -= 1;
                return Some(node_to_delete.data);
            }
        } else {
            if predicate(&self.head.as_ref()?.data) {
                let head = self.head.take()?;
                self.head = head.next_node;
                self.len -= 1;
                return Some(head.data);
            }
        }
        None
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next_node;
            self.len -= 1;
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

    pub fn contains_match<F>(&self, predicate: &F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        return if let Some(mut node) = self.head.as_ref() {
            while !predicate(&node.data) {
                if let Some(ref next) = node.next_node {
                    node = next;
                } else {
                    return false;
                }
            }
            true
        } else {
            false
        };
    }

    fn get_prev_node(&mut self, value: &T) -> Option<&mut Box<LinkedListNode<T>>> {
        return if let Some(ref mut head) = self.head {
            let mut node = head;
            while let Some(ref next) = node.next_node {
                if &next.data == value {
                    return Some(node);
                } else {
                    node = node.next_node.as_mut().unwrap();
                }
            }
            None
        } else {
            None
        };
    }

    fn get_prev_node_match<F>(&mut self, predicate: &F) -> Option<&mut Box<LinkedListNode<T>>>
    where
        F: Fn(&T) -> bool,
    {
        return if let Some(ref mut head) = self.head {
            let mut node = head;
            while let Some(ref next) = node.next_node {
                if predicate(&next.data) {
                    return Some(node);
                } else {
                    node = node.next_node.as_mut().unwrap();
                }
            }
            None
        } else {
            None
        };
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<T: PartialEq> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for element in vec {
            list.add(element);
        }
        list
    }
}

pub struct IntoIter<T: PartialEq>(LinkedList<T>);

impl<T: PartialEq> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: PartialEq> {
    next: Option<&'a LinkedListNode<T>>,
}

impl<'a, T: PartialEq> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next_node.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

pub struct IterMut<'a, T: PartialEq> {
    next: Option<&'a mut LinkedListNode<T>>,
}

impl<'a, T: PartialEq> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next_node.as_mut().map(|node| &mut **node);
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, LinkedListNode};

    #[test]
    fn from() {
        let mut list = LinkedList::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(list.pop().unwrap(), 1);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.pop().unwrap(), 5);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn get_tail() {
        let empty: LinkedList<i32> = LinkedList::new();
        assert_eq!(empty.get_tail(), None);
        let has_tail = LinkedList::from(vec![1, 2]);
        assert_eq!(has_tail.get_tail().unwrap(), &2);
        let head_only = LinkedList::from(vec![1]);
        assert_eq!(head_only.get_tail().unwrap(), &1);
        let deep_tail = LinkedList::from(vec![1, 2, 3, 4, 5]);
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
    fn delete() {
        let mut deep_list = LinkedList::from(vec![1, 2, 3, 4, 5]);
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
        let mut deep_list = LinkedList::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(deep_list.len(), 5);
        assert_eq!(deep_list.delete_match(|value| value == &3).unwrap(), 3);
        assert_eq!(deep_list.len(), 4);
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
        assert_eq!(deep_list.delete_match(|value| value == &1).unwrap(), 1);
        assert_eq!(deep_list.len(), 3);
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
        assert_eq!(deep_list.delete_match(|value| value / 5 == 1).unwrap(), 5);
        assert_eq!(deep_list.len(), 2);
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
        assert_eq!(deep_list.delete_match(|value| value == &2).unwrap(), 2);
        assert_eq!(deep_list.len(), 1);
        assert_eq!(
            deep_list.head.as_ref().unwrap(),
            &Box::from(LinkedListNode {
                data: 4,
                next_node: None,
            })
        );
        assert_eq!(deep_list.delete_match(|value| value / 2 == 2).unwrap(), 4);
        assert_eq!(deep_list.len(), 0);
        assert_eq!(deep_list.get_head(), None);
    }

    #[test]
    fn pop() {
        let mut list = LinkedList::from(vec![1, 2, 3, 4, 5]);
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

    #[test]
    fn set_head() {
        let mut list = LinkedList::new();
        assert_eq!(list.get_head(), None);
        #[derive(PartialOrd, PartialEq, Debug)]
        struct SomeInnerObject {
            inner_string: String,
        }
        list.add(SomeInnerObject {
            inner_string: "head".to_string(),
        });
        list.add(SomeInnerObject {
            inner_string: "second".to_string(),
        });
        list.add(SomeInnerObject {
            inner_string: "third".to_string(),
        });
        list.add(SomeInnerObject {
            inner_string: "fourth".to_string(),
        });
        assert_eq!(list.get_head().unwrap().inner_string, "head");
        list.set_head(SomeInnerObject {
            inner_string: "new head".to_string(),
        });
        assert_eq!(list.get_head().unwrap().inner_string, "new head");
        // Pop head, change it and set back
        let mut head = list.pop().unwrap();
        head.inner_string = "changed head".to_string();
        list.set_head(head);
        assert_eq!(list.get_head().unwrap().inner_string, "changed head");
    }

    #[test]
    fn len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        list.add(1);
        assert_eq!(list.len(), 1);
        list.add(5);
        list.add(10);
        list.add(20);
        assert_eq!(list.len(), 4);
        list.pop();
        assert_eq!(list.len(), 3);
        list.pop();
        assert_eq!(list.len(), 2);
        list.pop();
        assert_eq!(list.len(), 1);
        list.pop();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn into_iter() {
        let list = LinkedList::from(vec![1, 2, 3, 4, 5]);
        let mut iter = list.into_iter();
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn iter() {
        let list = LinkedList::from(vec![1, 2, 3, 4, 5]);
        let mut iter = list.iter();
        assert_eq!(*iter.next().unwrap(), 1);
        assert_eq!(*iter.next().unwrap(), 2);
        assert_eq!(*iter.next().unwrap(), 3);
        assert_eq!(*iter.next().unwrap(), 4);
        assert_eq!(*iter.next().unwrap(), 5);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::from(vec![1, 2, 3, 4, 5]);
        let iter_mut = list.iter_mut();
        for list_elem in iter_mut {
            *list_elem *= 2;
        }
        assert_eq!(list, LinkedList::from(vec![2, 4, 6, 8, 10]));
    }
}
