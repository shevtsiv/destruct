use crate::vector_based::VectorBasedDataStructure;

#[derive(PartialEq)]
pub struct GraphNode<'a, T> {
    value: T,
    lines: Vec<&'a GraphNode<'a, T>>,
}

impl<'a, T: PartialEq> GraphNode<'a, T> {
    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, new_value: T) {
        self.value = new_value;
    }

    pub fn get_lines(&self) -> &Vec<&GraphNode<T>> {
        &self.lines
    }

    pub fn add_line(&mut self, node: &'a GraphNode<T>) {
        if !self.lines.contains(&node) {
            self.lines.push(node);
        }
    }

    pub fn remove_line(&mut self, node: &'a GraphNode<T>) {
        let index = self.lines.iter().position(|node_to_delete| node_to_delete == &node).unwrap();
        self.lines.remove(index);
    }

    pub fn new(value: T) -> Self {
        GraphNode { value, lines: Vec::new() }
    }

    pub fn new_with_lines(value: T, lines: Vec<&'a GraphNode<T>>) -> Self {
        GraphNode { value, lines }
    }
}

pub struct Graph<'a, T> {
    nodes: Vec<GraphNode<'a, T>>,
}

impl<'a, T: PartialEq> VectorBasedDataStructure<T> for Graph<'a, T> {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Graph { nodes: Vec::with_capacity(capacity) }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn capacity(&self) -> usize {
        self.nodes.capacity()
    }
}
