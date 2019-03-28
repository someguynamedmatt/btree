use std::clone::Clone;
use std::cmp::PartialOrd;
use std::vec::Vec;

#[derive(Debug)]
struct BTree<K> {
    root_nodes: Vec<Box<Node<K>>>,
    order: u32
}

impl<K> BTree<K> {
    fn new_empty(order: u32) -> BTree<K> {
        BTree { root_nodes: vec![], order: order }
    }

    fn new_init(values: Vec<K>, order: u32) -> BTree<K> {
        let new_node: Node<K> = Node::new(vec![], order, values);
        BTree { root_nodes: vec![Box::new(new_node)], order: order }
    }
}

trait Tree<K> {
    fn push_empty_node(&mut self);
    fn push_node_with_vals(&mut self, values: Vec<K>);
}

impl<K> Tree<K> for BTree<K> {
    fn push_empty_node(&mut self) {
        let empty_node: Node<K> = Node::new(vec![], self.order, vec![]);
        self.root_nodes.push(Box::new(empty_node));
    }
    fn push_node_with_vals(&mut self, values: Vec<K>) {
        let new_node: Node<K> = Node::new(vec![], self.order, values);
        self.root_nodes.push(Box::new(new_node));
    }
}

#[derive(Debug)]
struct Node<K> {
    child_nodes: Vec<Box<Node<K>>>,
    max_children: u32,
    values: Vec<K>
}

impl<K> Node<K> {
    fn new(child_nodes: Vec<Box<Node<K>>>, max_children: u32, values: Vec<K>) -> Node<K> {
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

fn main() {
    println!("B+Tree Implementation");
    let mut tree: BTree<u32> = BTree::new_empty(3);
    let tree2: BTree<u32> = BTree::new_init(vec![1,2,3], 3);
    tree.push_empty_node();

    println!("{:#?}", tree);
    println!("{:#?}", tree2);
}
