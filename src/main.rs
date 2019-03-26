use std::clone::Clone;
use std::cmp::PartialOrd;
use std::vec::Vec;

#[derive(Debug)]
struct BTree<K> {
    root_nodes: Vec<Box<Node<K>>>,
    values: Vec<K>
}

impl<K> BTree<K> {
    fn new(root_nodes: Vec<Box<Node<K>>>, values: Vec<K>) -> BTree<K> {
        BTree { root_nodes: root_nodes, values: values }
    }
}

#[derive(Debug)]
struct Node<K> {
    child_nodes: Vec<Box<Node<K>>>,
    values: Vec<K>
}

impl<K> Node<K> {
    fn new(child_nodes: Vec<Box<Node<K>>>, values: Vec<K>) -> Node<K> {
        Node { child_nodes: child_nodes, values: values }
    }
}

trait TreeNode<K> {
    fn get_key(&self, key_index: K) -> Option<K>;
}

impl<K> TreeNode<K> for Node<K> where K: Clone + PartialOrd {
    fn get_key(&self, key_index: K) -> Option<K> {
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
    let mut tree: BTree<u32> = BTree::new(vec![], vec![1]);
    let mut n1: Node<u32> = Node::new(vec![], vec![234]);
    let mut n2: Node<u32> = Node::new(vec![], vec![2329234]);
    n2.child_nodes.push(Box::new(Node::new(vec![], vec![8])));
    tree.root_nodes.push(Box::new(n1));
    tree.root_nodes.push(Box::new(n2));
    println!("{:#?}", tree);
}
