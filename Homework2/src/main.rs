use rand::Rng;
use std::time::{Instant, Duration};
use term_table::{
    Table,
    TableStyle,
    row::Row,
    table_cell::{Alignment, TableCell},
};
mod binary_tree;

fn main() {
    let hundred = 100;
    let one_thousand = 1000;
    let ten_thousand = 10000;

    let mut binary_search_tree_100 = binary_tree::BinaryTree::new();
    let mut binary_search_tree_1k = binary_tree::BinaryTree::new();
    let mut binary_search_tree_10k = binary_tree::BinaryTree::new();

    population_unsorted(&mut binary_search_tree_100, hundred);
    population_unsorted(&mut binary_search_tree_1k, one_thousand);
    population_unsorted(&mut binary_search_tree_10k, ten_thousand);

    let tree_stats_100 = get_tree_stats(&binary_search_tree_100);
    let tree_stats_1k = get_tree_stats(&binary_search_tree_1k);
    let tree_stats_10k = get_tree_stats(&binary_search_tree_10k);

    let time_counts_of_bst_100 = calc_execution_times(&mut binary_search_tree_100);
    let time_counts_of_bst_1k = calc_execution_times(&mut binary_search_tree_1k);
    let time_counts_of_bst_10k = calc_execution_times(&mut binary_search_tree_10k);

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

    print_table("Unsorted Trees", unsorted_bts_entry);

    let mut binary_search_sorted_tree_100 = binary_tree::BinaryTree::new();
    let mut binary_search_sorted_tree_1k = binary_tree::BinaryTree::new();
    let mut binary_search_sorted_tree_10k = binary_tree::BinaryTree::new();

    population_sorted(&mut binary_search_sorted_tree_100, hundred);
    population_sorted(&mut binary_search_sorted_tree_1k, one_thousand);
    population_sorted(&mut binary_search_sorted_tree_10k, ten_thousand);

    let sorted_tree_stats_100 = get_tree_stats(&binary_search_sorted_tree_100);
    let sorted_tree_stats_1k = get_tree_stats(&binary_search_sorted_tree_1k);
    let sorted_tree_stats_10k = get_tree_stats(&binary_search_sorted_tree_10k);

    let time_counts_of_sorted_bst_100 = calc_execution_times(&mut binary_search_sorted_tree_100);
    let time_counts_of_sorted_bst_1k = calc_execution_times(&mut binary_search_sorted_tree_1k);
    let time_counts_of_sorted_bst_10k = calc_execution_times(&mut binary_search_sorted_tree_10k);

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

    print_table("Sorted Trees", sorted_bts_entry);
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
    let insert_time_100_str = "Insert Time: ".to_owned() + &insert_time_100 + " s ";
    let insert_time_1k = table_entries.from_1k_elements_tree.times.insert.as_secs_f64().to_string();
    let insert_time_1k_str = "Insert Time: ".to_owned() + &insert_time_1k + " s ";
    let insert_time_10k = table_entries.from_10k_elements_tree.times.insert.as_secs_f64().to_string();
    let insert_time_10k_str = "Insert Time: ".to_owned() + &insert_time_10k + " s ";
    table.add_row(Row::new(vec![
                TableCell::new(insert_time_100_str),
            TableCell::new_with_alignment(insert_time_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(insert_time_10k_str, 2, Alignment::Left)
    ]));

    let size_time_100 = table_entries.from_100_elements_tree.times.size.as_secs_f64().to_string();
    let size_time_100_str = "Size Time: ".to_owned() + &size_time_100 + " s ";
    let size_data_100 = table_entries.from_100_elements_tree.stats.size.to_string();
    let size_entry_100_str = size_time_100_str + "\nSize of Tree: " + &size_data_100;

    let size_time_1k = table_entries.from_1k_elements_tree.times.size.as_secs_f64().to_string();
    let size_data_1k = table_entries.from_1k_elements_tree.stats.size.to_string();
    let size_time_1k_str = "Size Time: ".to_owned() + &size_time_1k + " s ";
    let size_entry_1k_str = size_time_1k_str + "\nSize of Tree: " + &size_data_1k;

    let size_time_10k = table_entries.from_10k_elements_tree.times.size.as_secs_f64().to_string();
    let size_data_10k = table_entries.from_10k_elements_tree.stats.size.to_string();
    let size_time_10k_str = "Size Time: ".to_owned() + &size_time_10k + " s ";
    let size_entry_10k_str = size_time_10k_str + "\nSize of Tree: " + &size_data_10k;

    table.add_row(Row::new(vec![
            TableCell::new(size_entry_100_str),
            TableCell::new_with_alignment(size_entry_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(size_entry_10k_str, 2, Alignment::Left)
    ]));

    let depth_time_100 = table_entries.from_100_elements_tree.times.depth.as_secs_f64().to_string();
    let depth_time_100_str = "Depth Time: ".to_owned() + &depth_time_100 + " s ";
    let depth_data_100 = table_entries.from_100_elements_tree.stats.depth.to_string();
    let depth_entry_100_str = depth_time_100_str + "\nDepth of Tree: " + &depth_data_100;

    let depth_time_1k = table_entries.from_1k_elements_tree.times.depth.as_secs_f64().to_string();
    let depth_data_1k = table_entries.from_1k_elements_tree.stats.depth.to_string();
    let depth_time_1k_str = "Depth Time: ".to_owned() + &depth_time_1k + " s ";
    let depth_entry_1k_str = depth_time_1k_str + "\nDepth of Tree: " + &depth_data_1k;

    let depth_time_10k = table_entries.from_10k_elements_tree.times.depth.as_secs_f64().to_string();
    let depth_data_10k = table_entries.from_10k_elements_tree.stats.depth.to_string();
    let depth_time_10k_str = "Depth Time: ".to_owned() + &depth_time_10k + " s ";
    let depth_entry_10k_str = depth_time_10k_str + "\nDepth of Tree: " + &depth_data_10k;

    table.add_row(Row::new(vec![
            TableCell::new(depth_entry_100_str),
            TableCell::new_with_alignment(depth_entry_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(depth_entry_10k_str, 2, Alignment::Left)
    ]));

    let search_time_100 = table_entries.from_100_elements_tree.times.search.as_secs_f64().to_string();
    let search_time_100_str = "Search Time: ".to_owned() + &search_time_100 + " s ";
    let search_time_1k = table_entries.from_1k_elements_tree.times.search.as_secs_f64().to_string();
    let search_time_1k_str = "Search Time: ".to_owned() + &search_time_1k + " s ";
    let search_time_10k = table_entries.from_10k_elements_tree.times.search.as_secs_f64().to_string();
    let search_time_10k_str = "Search Time: ".to_owned() + &search_time_10k + " s ";
    table.add_row(Row::new(vec![
            TableCell::new(search_time_100_str),
            TableCell::new_with_alignment(search_time_1k_str, 1, Alignment::Left),
            TableCell::new_with_alignment(search_time_10k_str, 2, Alignment::Left)
    ]));

    println!("{}", table.render());
}
