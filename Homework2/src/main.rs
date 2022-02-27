use rand::Rng;
use std::time::Instant;
use term_table::{
    Table,
    TableStyle,
    row::Row,
    table_cell::{Alignment, TableCell},
};
pub mod binary_tree;
pub mod benchmarks;
use crate::benchmarks::TimeCounts;

fn main() {
    let unsorted_averages_table_entry = run_unsorted_test(5);
    print_table("Unsorted Binary Tree Tests Average", unsorted_averages_table_entry);
    
    let sorted_averages_table_entry = run_sorted_test(5);
    print_table("Sorted Binary Tree Tests Average", sorted_averages_table_entry);

}

fn population_unsorted(tree: &mut binary_tree::BinaryTree, elements_count: u64) {
    let mut rng = rand::thread_rng();

    for _n in 0..elements_count {
        tree.insert(rng.gen_range(0.0..1000.0));
    }
}

fn population_sorted(tree: &mut binary_tree::BinaryTree, elements_count: u64) {
    let mut rng = rand::thread_rng();
    let mut number_vec: Vec<f64> = Vec::new();

    for _n in 0..elements_count {
        number_vec.push(rng.gen_range(0.0..1000.0));
    }

    // sorts vector of floats f64s from loweset to highest
    number_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for (_i, n) in number_vec.iter().enumerate() {
        tree.insert(*n);
    }

}

struct BinaryTreeStats {
    size: u64,
    depth: u64,
}

impl Copy for BinaryTreeStats {}
impl Clone for BinaryTreeStats {
    fn clone(&self) -> Self {
        *self
    }
}
fn get_tree_stats(tree: &binary_tree::BinaryTree) -> BinaryTreeStats {
    BinaryTreeStats {
        size: tree.size(),
        depth: tree.depth(),
    }
}

fn calc_execution_times(tree: &mut binary_tree::BinaryTree) -> benchmarks::TimeCounts {
    // Caculations the time of the size() method
    let start_timer = Instant::now();
    let tree_size = tree.size();
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
    tree.search(new_element);
    let end_timer = Instant::now();
    let search_time = end_timer.duration_since(start_timer);

    // Caculations the time of the depth() method
    let start_timer = Instant::now();
    tree.depth();
    let end_timer = Instant::now();
    let depth_time = end_timer.duration_since(start_timer);

    // Caculations the time of the remove() method
    let start_timer = Instant::now();
    tree.remove(new_element);
    let end_timer = Instant::now();
    let remove_time = end_timer.duration_since(start_timer);

    TimeCounts {
        size: size_time,
        insert: insert_time,
        search: search_time,
        remove: remove_time,
        depth: depth_time,
    }
}

struct BinaryTreeData {
    stats: BinaryTreeStats,
    times: TimeCounts,
}

struct TableEntry {
    from_100_elements_tree: BinaryTreeData,
    from_1k_elements_tree: BinaryTreeData,
    from_10k_elements_tree: BinaryTreeData,
}

fn print_table(title: &str, table_entries: TableEntry) {
    let mut table = Table::new();
    table.max_column_width = 150;

    table.style = TableStyle::extended();

    table.add_row(Row::new(vec![
            TableCell::new_with_alignment(title, 4, Alignment::Center)
    ]));

    table.add_row(Row::new(vec![
            TableCell::new("100 Elements"),
            TableCell::new_with_alignment("1k Elements", 1, Alignment::Left),
            TableCell::new_with_alignment("10k Elements", 2, Alignment::Left)
    ]));

    let insert_time_100 = table_entries.from_100_elements_tree.times.insert.as_secs_f64().to_string();
    let insert_time_100_str = "Insert Time: ".to_owned() + &insert_time_100 + "s";
    let insert_time_1k = table_entries.from_1k_elements_tree.times.insert.as_secs_f64().to_string();
    let insert_time_1k_str = "Insert Time: ".to_owned() + &insert_time_1k + "s";
    let insert_time_10k = table_entries.from_10k_elements_tree.times.insert.as_secs_f64().to_string();
    let insert_time_10k_str = "Insert Time: ".to_owned() + &insert_time_10k + "s";
    table.add_row(Row::new(vec![
                TableCell::new(insert_time_100_str),
            TableCell::new_with_alignment(insert_time_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(insert_time_10k_str, 2, Alignment::Left)
    ]));

    let size_time_100 = table_entries.from_100_elements_tree.times.size.as_secs_f64().to_string();
    let size_time_100_str = "Size Time: ".to_owned() + &size_time_100 + "s";
    let size_data_100 = table_entries.from_100_elements_tree.stats.size.to_string();
    let size_entry_100_str = size_time_100_str + "\nSize of Tree: " + &size_data_100;

    let size_time_1k = table_entries.from_1k_elements_tree.times.size.as_secs_f64().to_string();
    let size_data_1k = table_entries.from_1k_elements_tree.stats.size.to_string();
    let size_time_1k_str = "Size Time: ".to_owned() + &size_time_1k + "s";
    let size_entry_1k_str = size_time_1k_str + "\nSize of Tree: " + &size_data_1k;

    let size_time_10k = table_entries.from_10k_elements_tree.times.size.as_secs_f64().to_string();
    let size_data_10k = table_entries.from_10k_elements_tree.stats.size.to_string();
    let size_time_10k_str = "Size Time: ".to_owned() + &size_time_10k + "s";
    let size_entry_10k_str = size_time_10k_str + "\nSize of Tree: " + &size_data_10k;

    table.add_row(Row::new(vec![
            TableCell::new(size_entry_100_str),
            TableCell::new_with_alignment(size_entry_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(size_entry_10k_str, 2, Alignment::Left)
    ]));

    let depth_time_100 = table_entries.from_100_elements_tree.times.depth.as_secs_f64().to_string();
    let depth_time_100_str = "Depth Time: ".to_owned() + &depth_time_100 + "s";
    let depth_data_100 = table_entries.from_100_elements_tree.stats.depth.to_string();
    let depth_entry_100_str = depth_time_100_str + "\nDepth of Tree: " + &depth_data_100;

    let depth_time_1k = table_entries.from_1k_elements_tree.times.depth.as_secs_f64().to_string();
    let depth_data_1k = table_entries.from_1k_elements_tree.stats.depth.to_string();
    let depth_time_1k_str = "Depth Time: ".to_owned() + &depth_time_1k + "s";
    let depth_entry_1k_str = depth_time_1k_str + "\nDepth of Tree: " + &depth_data_1k;

    let depth_time_10k = table_entries.from_10k_elements_tree.times.depth.as_secs_f64().to_string();
    let depth_data_10k = table_entries.from_10k_elements_tree.stats.depth.to_string();
    let depth_time_10k_str = "Depth Time: ".to_owned() + &depth_time_10k + "s";
    let depth_entry_10k_str = depth_time_10k_str + "\nDepth of Tree: " + &depth_data_10k;

    table.add_row(Row::new(vec![
            TableCell::new(depth_entry_100_str),
            TableCell::new_with_alignment(depth_entry_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(depth_entry_10k_str, 2, Alignment::Left)
    ]));

    let search_time_100 = table_entries.from_100_elements_tree.times.search.as_secs_f64().to_string();
    let search_time_100_str = "Search Time: ".to_owned() + &search_time_100 + "s";
    let search_time_1k = table_entries.from_1k_elements_tree.times.search.as_secs_f64().to_string();
    let search_time_1k_str = "Search Time: ".to_owned() + &search_time_1k + "s";
    let search_time_10k = table_entries.from_10k_elements_tree.times.search.as_secs_f64().to_string();
    let search_time_10k_str = "Search Time: ".to_owned() + &search_time_10k + "s";
    table.add_row(Row::new(vec![
            TableCell::new(search_time_100_str),
            TableCell::new_with_alignment(search_time_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(search_time_10k_str, 2, Alignment::Left)
    ]));

    let remove_time_100 = table_entries.from_100_elements_tree.times.remove.as_secs_f64().to_string();
    let remove_time_100_str = "Remove Time: ".to_owned() + &remove_time_100 + "s";
    let remove_time_1k = table_entries.from_1k_elements_tree.times.remove.as_secs_f64().to_string();
    let remove_time_1k_str = "Remove Time: ".to_owned() + &remove_time_1k + "s";
    let remove_time_10k = table_entries.from_10k_elements_tree.times.remove.as_secs_f64().to_string();
    let remove_time_10k_str = "Remove Time: ".to_owned() + &remove_time_10k + "s";
    table.add_row(Row::new(vec![
            TableCell::new(remove_time_100_str),
            TableCell::new_with_alignment(remove_time_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(remove_time_10k_str, 2, Alignment::Left)
    ]));

    println!("{}", table.render());
}

fn run_unsorted_test(run_times: u8) -> TableEntry {
    let mut unsorted_time_counts_100_vec = Vec::<benchmarks::TimeCounts>::new();
    let mut unsorted_time_counts_1k_vec = Vec::<benchmarks::TimeCounts>::new();
    let mut unsorted_time_counts_10k_vec = Vec::<benchmarks::TimeCounts>::new();

    for _x in 0..run_times {
        let mut binary_search_tree_100 = binary_tree::BinaryTree::new();
        let mut binary_search_tree_1k = binary_tree::BinaryTree::new();
        let mut binary_search_tree_10k = binary_tree::BinaryTree::new();

        population_unsorted(&mut binary_search_tree_100, 100);
        population_unsorted(&mut binary_search_tree_1k, 1000);
        population_unsorted(&mut binary_search_tree_10k, 10000);

        let tree_stats_100 = get_tree_stats(&binary_search_tree_100);
        let tree_stats_1k = get_tree_stats(&binary_search_tree_1k);
        let tree_stats_10k = get_tree_stats(&binary_search_tree_10k);

        let time_counts_of_bst_100 = calc_execution_times(&mut binary_search_tree_100);
        unsorted_time_counts_100_vec.push(time_counts_of_bst_100.clone());
        let time_counts_of_bst_1k = calc_execution_times(&mut binary_search_tree_1k);
        unsorted_time_counts_1k_vec.push(time_counts_of_bst_1k.clone());
        let time_counts_of_bst_10k = calc_execution_times(&mut binary_search_tree_10k);
        unsorted_time_counts_10k_vec.push(time_counts_of_bst_10k.clone());

        let unsorted_bts_entry = TableEntry {
            from_100_elements_tree: 
                BinaryTreeData {
                    stats: tree_stats_100,
                    times: time_counts_of_bst_100,
                },
            from_1k_elements_tree:
                BinaryTreeData {
                    stats: tree_stats_1k,
                    times: time_counts_of_bst_1k,
                },
            from_10k_elements_tree:
                BinaryTreeData {
                    stats: tree_stats_10k,
                    times: time_counts_of_bst_10k,
                },
        };

        print_table("Unsorted Tree Test", unsorted_bts_entry);
    }

    let time_counts_100_average = benchmarks::calc_average_time_counts(&mut unsorted_time_counts_100_vec);
    let time_counts_1k_average = benchmarks::calc_average_time_counts(&mut unsorted_time_counts_1k_vec);
    let time_counts_10k_average = benchmarks::calc_average_time_counts(&mut unsorted_time_counts_10k_vec);

    let dummy_stats = BinaryTreeStats {
        size: 0,
        depth: 0,
    };

    TableEntry {
        from_100_elements_tree: 
            BinaryTreeData {
                stats: dummy_stats,
                times: time_counts_100_average,
            },
        from_1k_elements_tree:
            BinaryTreeData {
                stats: dummy_stats,
                times: time_counts_1k_average,
            },
        from_10k_elements_tree:
            BinaryTreeData {
                stats: dummy_stats,
                times: time_counts_10k_average,
            },
    }
}

fn run_sorted_test(run_times: u8) -> TableEntry {
    let mut sorted_time_counts_100_vec = Vec::<benchmarks::TimeCounts>::new();
    let mut sorted_time_counts_1k_vec = Vec::<benchmarks::TimeCounts>::new();
    let mut sorted_time_counts_10k_vec = Vec::<benchmarks::TimeCounts>::new();

    for _x in 0..run_times {
        let mut binary_search_sorted_tree_100 = binary_tree::BinaryTree::new();
        let mut binary_search_sorted_tree_1k = binary_tree::BinaryTree::new();
        let mut binary_search_sorted_tree_10k = binary_tree::BinaryTree::new();

        population_sorted(&mut binary_search_sorted_tree_100, 100);
        population_sorted(&mut binary_search_sorted_tree_1k, 1000);
        population_sorted(&mut binary_search_sorted_tree_10k, 10000);

        let sorted_tree_stats_100 = get_tree_stats(&binary_search_sorted_tree_100);
        let sorted_tree_stats_1k = get_tree_stats(&binary_search_sorted_tree_1k);
        let sorted_tree_stats_10k = get_tree_stats(&binary_search_sorted_tree_10k);

        let time_counts_of_sorted_bst_100 = calc_execution_times(&mut binary_search_sorted_tree_100);
        sorted_time_counts_100_vec.push(time_counts_of_sorted_bst_100.clone());
        let time_counts_of_sorted_bst_1k = calc_execution_times(&mut binary_search_sorted_tree_1k);
        sorted_time_counts_1k_vec.push(time_counts_of_sorted_bst_1k.clone());
        let time_counts_of_sorted_bst_10k = calc_execution_times(&mut binary_search_sorted_tree_10k);
        sorted_time_counts_10k_vec.push(time_counts_of_sorted_bst_10k.clone());

        let sorted_bts_entry = TableEntry {
            from_100_elements_tree: 
                BinaryTreeData {
                    stats: sorted_tree_stats_100,
                    times: time_counts_of_sorted_bst_100,
                },
            from_1k_elements_tree:
                BinaryTreeData {
                    stats: sorted_tree_stats_1k,
                    times: time_counts_of_sorted_bst_1k,
                },
            from_10k_elements_tree:
                BinaryTreeData {
                    stats: sorted_tree_stats_10k,
                    times: time_counts_of_sorted_bst_10k,
                },
        };

        print_table("Sorted Tree Test", sorted_bts_entry);
    }

    let time_counts_100_average = benchmarks::calc_average_time_counts(&mut sorted_time_counts_100_vec);
    let time_counts_1k_average = benchmarks::calc_average_time_counts(&mut sorted_time_counts_1k_vec);
    let time_counts_10k_average = benchmarks::calc_average_time_counts(&mut sorted_time_counts_10k_vec);

    let dummy_stats = BinaryTreeStats {
        size: 0,
        depth: 0,
    };

    TableEntry {
        from_100_elements_tree: 
            BinaryTreeData {
                stats: dummy_stats,
                times: time_counts_100_average,
            },
        from_1k_elements_tree:
            BinaryTreeData {
                stats: dummy_stats,
                times: time_counts_1k_average,
            },
        from_10k_elements_tree:
            BinaryTreeData {
                stats: dummy_stats,
                times: time_counts_10k_average,
            },
    }
}
