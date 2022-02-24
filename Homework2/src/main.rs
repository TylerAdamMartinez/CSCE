use rand::Rng;
use std::time::Instant;
mod binary_tree;

fn main() {
    let mut binary_search_tree = binary_tree::BinaryTree::new();

    populate_tree(&mut binary_search_tree, 10000);

    let start_timer = Instant::now();
    binary_search_tree.insert(1001.0);
    let end_timer = start_timer.elapsed();

    print_tree_stats(&binary_search_tree);
    println!("Find 1001.0 in the tree\nStatus: {:?}", binary_search_tree.search(&binary_search_tree, 1001.0));
    println!("Find 1055.0 in the tree\nStatus: {:?}", binary_search_tree.search(&binary_search_tree, 1055.0));

    println!("time duration: {}ms", end_timer.as_secs() * 1000);
}

fn populate_tree(tree: &mut binary_tree::BinaryTree, elements_count: u64) {
    let mut rng = rand::thread_rng();

    for _n in 1..elements_count {
        tree.insert(rng.gen_range(0.0..1000.0));
    }
}

fn print_tree_stats(tree: &binary_tree::BinaryTree) {
    println!("{:#?}", tree);
    println!("Number of elements: {}", tree.size(&tree));
    println!("Max depth of tree: {}", tree.depth(&tree));
}
