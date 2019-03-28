#[derive(Debug)]
pub struct Node<K> {
    child_nodes: Vec<Box<Node<K>>>,
    max_children: u32,
    values: Vec<K>
}

impl<K> Node<K> {
    pub fn new(child_nodes: Vec<Box<Node<K>>>, max_children: u32, values: Vec<K>) -> Node<K> {
        Node { child_nodes: child_nodes, max_children: max_children, values: values }
    }
}

trait TreeNode<K> {
    fn get_val_at_index(&self, key_index: K) -> Option<K>;
}

impl<K> TreeNode<K> for Node<K> where K: Clone + PartialOrd {
    fn get_val_at_index(&self, key_index: K) -> Option<K> {
        for (i, k) in self.values.iter().enumerate() {
            if key_index == *k {
                return Some(self.values[i].clone())
            }
        }
        None
    }
}
