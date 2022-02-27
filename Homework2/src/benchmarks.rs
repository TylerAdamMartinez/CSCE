use std::time::Duration;

#[derive(Debug)]
pub struct TimeCounts {
    pub insert: Duration,
    pub size: Duration,
    pub depth: Duration,
    pub search: Duration,
    pub remove: Duration,
}

impl Copy for TimeCounts {}
impl Clone for TimeCounts {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn calc_average_time_counts(time_counts_vec: &mut Vec<TimeCounts>) -> TimeCounts {
    let mut insert_average = Duration::new(0, 0);
    let mut size_average = Duration::new(0, 0);
    let mut depth_average = Duration::new(0, 0);
    let mut search_average = Duration::new(0, 0);
    let mut remove_average = Duration::new(0, 0);

    for time_count in time_counts_vec.iter() {
        insert_average = time_count.insert;
    }
    insert_average = insert_average / time_counts_vec.len().try_into().unwrap();

    for time_count in time_counts_vec.iter() {
        size_average = time_count.size;
    }
    size_average = size_average / time_counts_vec.len().try_into().unwrap();

    for time_count in time_counts_vec.iter() {
        depth_average = time_count.depth;
    }
    depth_average = depth_average / time_counts_vec.len().try_into().unwrap();

    for time_count in time_counts_vec.iter() {
        search_average = time_count.search;
    }
    search_average = search_average / time_counts_vec.len().try_into().unwrap();

    for time_count in time_counts_vec.iter() {
        remove_average = time_count.remove;
    }
    remove_average = remove_average / time_counts_vec.len().try_into().unwrap();

    TimeCounts {
        insert: insert_average,
        size: size_average,
        depth: depth_average,
        search: search_average,
        remove: remove_average,
    }
}
