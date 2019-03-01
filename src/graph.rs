use crate::vector_based::VectorBasedDataStructure;

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::vector_based::VectorBasedDataStructure;
    use crate::graph::Graph;

    #[test]
    fn new() {
        let graph: Graph<i32> = Graph::new();
        assert_eq!(graph.len(), 0);
        assert_eq!(graph.capacity(), 0);
    }

    #[test]
    fn with_capacity() {
        let graph: Graph<i32> = Graph::with_capacity(10);
        assert_eq!(graph.len(), 0);
        assert_eq!(graph.capacity(), 10);
    }

    use crate::graph::GraphNode;

    #[test]
    fn get_and_set_value() {
        let mut graph_node = GraphNode::new(5);
        assert_eq!(graph_node.get_value(), &5);
        graph_node.set_value(10);
        assert_eq!(graph_node.get_value(), &10);
    }

    #[test]
    fn new_with_lines_and_get_lines() {
        let first_node = GraphNode::new(1);
        let second_node = GraphNode::new(2);
        let third_node = GraphNode::new(3);
        let lines = vec![&first_node, &second_node, &third_node];
        let graph_node = GraphNode::new_with_lines(0, lines);
        assert_eq!(graph_node.get_lines(), &vec![&GraphNode::new(1), &GraphNode::new(2), &GraphNode::new(3)]);
    }

    #[test]
    fn add_line() {
        let mut graph_node = GraphNode::new(0);
        assert!(graph_node.get_lines().is_empty());
        let another_graph_node = GraphNode::new(1);
        graph_node.add_line(&another_graph_node);
        assert_eq!(graph_node.get_lines(), &vec![&another_graph_node]);
    }

    #[test]
    fn remove_line() {
        let first_node = GraphNode::new(1);
        let second_node = GraphNode::new(2);
        let third_node = GraphNode::new(3);
        let lines = vec![&first_node, &second_node, &third_node];
        let mut graph_node = GraphNode::new_with_lines(0, lines);
        assert_eq!(graph_node.get_lines(), &vec![&GraphNode::new(1), &GraphNode::new(2), &GraphNode::new(3)]);
        graph_node.remove_line(&second_node);
        assert_eq!(graph_node.get_lines(), &vec![&GraphNode::new(1), &GraphNode::new(3)]);
        graph_node.remove_line(&third_node);
        assert_eq!(graph_node.get_lines(), &vec![&GraphNode::new(1)]);
        graph_node.remove_line(&first_node);
        assert!(graph_node.get_lines().is_empty());
    }
}
