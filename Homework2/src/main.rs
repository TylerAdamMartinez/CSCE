use rand::Rng;
mod binary_tree;

fn main() {
    let mut binary_search_tree = binary_tree::BinaryTree::new();
    let mut rng = rand::thread_rng();

    for _n in 0..10 {
        binary_search_tree.insert(rng.gen_range(0.0..1000.0));
    }

    binary_search_tree.insert(1001.0);

    println!("binary_search_tree is {:#?}", binary_search_tree);
    println!("binary_search_tree has {} elements", binary_search_tree.size(&binary_search_tree));
    println!("binary_search_tree has a depth of {}", binary_search_tree.depth(&binary_search_tree));
    println!("Find 1001.0 in the tree\nStatus: {:?}", binary_search_tree.search(&binary_search_tree, 1001.0));
    println!("Find 1055.0 in the tree\nStatus: {:?}", binary_search_tree.search(&binary_search_tree, 1055.0));

}
