use rand::Rng;
use std::time::{Instant, Duration};
mod binary_tree;

fn main() {
    let mut binary_search_tree_100 = binary_tree::BinaryTree::new();
    let mut binary_search_tree_1k = binary_tree::BinaryTree::new();
    let mut binary_search_tree_10k = binary_tree::BinaryTree::new();

    let hundred = 100;
    let one_thousand = 1000;
    let ten_thousand = 10000;

    populate_tree_rand(&mut binary_search_tree_100, hundred);
    populate_tree_rand(&mut binary_search_tree_1k, one_thousand);
    populate_tree_rand(&mut binary_search_tree_10k, ten_thousand);

    let tree_stats_100 = get_tree_stats(&binary_search_tree_100);
    let tree_stats_1k = get_tree_stats(&binary_search_tree_1k);
    let tree_stats_10k = get_tree_stats(&binary_search_tree_10k);

    let time_counts_of_bst_100 = calc_execution_times(&mut binary_search_tree_100);
    let time_counts_of_bst_1k = calc_execution_times(&mut binary_search_tree_1k);
    let time_counts_of_bst_10k = calc_execution_times(&mut binary_search_tree_10k);

    println!("insert: {:?}", time_counts_of_bst_100.insert);
    println!("search: {:?}", time_counts_of_bst_100.search);
    println!("size: {:?}", time_counts_of_bst_100.size);
    println!("depth: {:?}", time_counts_of_bst_100.depth);
}

fn populate_tree_rand(tree: &mut binary_tree::BinaryTree, elements_count: u64) {
    let mut rng = rand::thread_rng();

    for _n in 1..elements_count {
        tree.insert(rng.gen_range(0.0..1000.0));
    }
}

struct BinaryTreeStats {
    size: u64,
    depth: u64,
}

fn get_tree_stats(tree: &binary_tree::BinaryTree) -> BinaryTreeStats {
    BinaryTreeStats {
        size: tree.size(&tree),
        depth: tree.depth(&tree),
    }
}

struct TimeCounts {
    insert: Duration,
    size: Duration,
    depth: Duration,
    search: Duration,
}

fn calc_execution_times(tree: &mut binary_tree::BinaryTree) -> TimeCounts {
    // Caculations the time of the size() method
    let start_timer = Instant::now();
    let tree_size = tree.size(&tree);
    let end_timer = Instant::now();
    let size_time = end_timer.duration_since(start_timer);

    let new_element = tree_size as f64 + 1.0;

    // Caculations the time of the insert() method
    let start_timer = Instant::now();
    tree.insert(new_element);
    let end_timer = Instant::now();
    let insert_time = end_timer.duration_since(start_timer);

    // Caculations the time of the search() method
    let start_timer = Instant::now();
    tree.search(&tree, new_element);
    let end_timer = Instant::now();
    let search_time = end_timer.duration_since(start_timer);

    // Caculations the time of the depth() method
    let start_timer = Instant::now();
    tree.depth(&tree);
    let end_timer = Instant::now();
    let depth_time = end_timer.duration_since(start_timer);

    TimeCounts {
        size: size_time,
        insert: insert_time,
        search: search_time,
        depth: depth_time,
    }
}

