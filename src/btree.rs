use super::node::Node;

#[derive(Debug)]
pub struct BTree<K> {
    root_nodes: Vec<Box<Node<K>>>,
    order: u32
}

impl<K> BTree<K> {
    pub fn new_empty(order: u32) -> BTree<K> {
        BTree { root_nodes: vec![], order: order }
    }

    pub fn new_init(values: Vec<K>, order: u32) -> BTree<K> {
        let new_node: Node<K> = Node::new(vec![], order, values);
        BTree { root_nodes: vec![Box::new(new_node)], order: order }
    }

    pub fn push_empty_node(&mut self) {
        let empty_node: Node<K> = Node::new(vec![], self.order, vec![]);
        self.root_nodes.push(Box::new(empty_node));
    }

    pub fn push_node_with_vals(&mut self, values: Vec<K>) {
        let new_node: Node<K> = Node::new(vec![], self.order, values);
        self.root_nodes.push(Box::new(new_node));
    }
}
