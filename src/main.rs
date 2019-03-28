use btree::btree::{BTree};

fn main() {
    println!("B+Tree Implementation");
    let mut tree: BTree<u32> = BTree::new_empty(3);
    let tree2: BTree<u32> = BTree::new_init(vec![1,2,3], 3);
    tree.push_empty_node();

    println!("{:#?}", tree);
    println!("{:#?}", tree2);
}
