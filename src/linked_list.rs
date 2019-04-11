use std::rc::Rc;

pub struct LinkedListNode<T> {
    data: Option<T>,
    next_node: Option<Rc<LinkedListNode<T>>>,
}

pub struct LinkedList<T> {
    head: Option<LinkedListNode<T>>,
}

impl<T> LinkedListNode<T> {
    pub fn get_data(&self) -> &Option<T> {
        &self.data
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
}

impl<T> LinkedList<T> {
    pub fn get_head(&self) -> Option<&LinkedListNode<T>> {
        self.head.as_ref()
    }

    pub fn get_head_mut(&mut self) -> Option<&mut LinkedListNode<T>> {
        self.head.as_mut()
    }

    pub fn new() -> Self {
        LinkedList { head: None }
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::LinkedList;

    #[test]
    fn new() {
        let _linked_list: LinkedList<i32> = LinkedList::new();
    }
}
