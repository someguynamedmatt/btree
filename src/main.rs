use btree::btree::{BTree};

fn main() {
    println!("BTree Implementation\n");

    // Initialize an empty tree with zero nodes
    let tree: BTree<u32> = BTree::new_empty(3);

    // Initialize an empty tree with zero nodes, then push 2 empty nodes into it
    let mut tree_push: BTree<u32> = BTree::new_empty(3);
    tree_push.push_empty_node();
    tree_push.push_empty_node();

    // Initialize the tree with a single node and defined values
    let tree2: BTree<u32> = BTree::new_init(vec![1,2,3], 3);

    // Generics give it flexibility
    let tree3: BTree<String> = BTree::new_init(vec!["hello".to_string(), "world".to_string()], 2);

    println!("{} {:#?} {}", "Empty Tree, zero nodes:\n", tree, "\n");
    println!("{} {:#?} {}", "New Tree, then 2 nodes pushed:\n", tree_push, "\n");
    println!("{} {:#?} {}", "Tree with a single node and defined values:\n", tree2, "\n");
    println!("{} {:#?} {}", "Tree with a single node, defined values, demonstrating generics:\n", tree3, "\n");
}
