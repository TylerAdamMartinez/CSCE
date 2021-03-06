use std::time::Duration;
use std::iter::Sum;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct TimeCounts {
    pub insert: Duration,
    pub size: Duration,
    pub depth: Duration,
    pub search: Duration,
    pub remove: Duration,
}

impl TimeCounts {
    fn new() -> Self {
        Self {
            insert: Duration::ZERO,
            size: Duration::ZERO,
            depth: Duration::ZERO,
            search: Duration::ZERO,
            remove: Duration::ZERO,
        }
    }
}

impl Copy for TimeCounts {}
impl Clone for TimeCounts {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Sum<&'a Self> for TimeCounts {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = &'a Self> {
        iter.fold(Self {
            insert: Duration::ZERO,
            size: Duration::ZERO,
            depth: Duration::ZERO,
            search: Duration::ZERO,
            remove: Duration::ZERO,
        }, |a, b| Self {
            insert: a.insert + b.insert,
            size: a.size + b.size,
            depth: a.depth + b.depth,
            search: a.search + b.search,
            remove: a.remove + b.remove,
        })
    }
}

fn calc_time_counts_sum(time_counts_vec: &Vec<TimeCounts>) -> TimeCounts {
    Iterator::sum(time_counts_vec.iter())
}

pub fn calc_average_time_counts(time_counts_vec: &Vec<TimeCounts>) -> TimeCounts {
    let mut time_counts_sums = calc_time_counts_sum(&time_counts_vec);

    time_counts_sums.insert = time_counts_sums.insert / time_counts_vec.len().try_into().unwrap();
    time_counts_sums.size = time_counts_sums.size / time_counts_vec.len().try_into().unwrap();
    time_counts_sums.depth = time_counts_sums.depth / time_counts_vec.len().try_into().unwrap();
    time_counts_sums.search = time_counts_sums.search / time_counts_vec.len().try_into().unwrap();
    time_counts_sums.remove = time_counts_sums.remove / time_counts_vec.len().try_into().unwrap();

    time_counts_sums
}

pub fn calc_worst_time_counts(time_counts_vec: &Vec<TimeCounts>) -> TimeCounts {
    let mut time_counts_worst_case = TimeCounts::new();
    let mut max_insert = Duration::ZERO;
    let mut max_size = Duration::ZERO;
    let mut max_depth = Duration::ZERO;
    let mut max_search = Duration::ZERO;
    let mut max_remove = Duration::ZERO;
    
    for current_max in time_counts_vec.iter() {
        if current_max.insert > max_insert {
            max_insert = current_max.insert;
        }
        if current_max.size > max_size {
            max_size = current_max.size;
        }
        if current_max.depth > max_depth {
            max_depth = current_max.depth;
        }
        if current_max.search > max_search {
            max_search = current_max.search;
        }
        if current_max.remove > max_remove {
            max_remove = current_max.remove;
        }
    }

    time_counts_worst_case.insert = max_insert;
    time_counts_worst_case.size = max_size;
    time_counts_worst_case.depth = max_depth;
    time_counts_worst_case.search = max_search;
    time_counts_worst_case.remove = max_remove;

    time_counts_worst_case
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_time_counts_sum() {
        let mut test_vec = Vec::<TimeCounts>::new();

        test_vec.push(TimeCounts {
            insert: Duration::from_secs(5) ,
            size: Duration::from_secs(5),
            depth: Duration::from_secs(5),
            search: Duration::from_secs(5),
            remove: Duration::from_secs(5),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(4) ,
            size: Duration::from_secs(4),
            depth: Duration::from_secs(4),
            search: Duration::from_secs(4),
            remove: Duration::from_secs(4),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(3) ,
            size: Duration::from_secs(3),
            depth: Duration::from_secs(3),
            search: Duration::from_secs(3),
            remove: Duration::from_secs(3),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(7) ,
            size: Duration::from_secs(7),
            depth: Duration::from_secs(7),
            search: Duration::from_secs(7),
            remove: Duration::from_secs(7),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(1) ,
            size: Duration::from_secs(1),
            depth: Duration::from_secs(1),
            search: Duration::from_secs(1),
            remove: Duration::from_secs(1),
        });

        assert_eq!(super::calc_time_counts_sum(&test_vec),
            TimeCounts {
            insert: Duration::from_secs(20) ,
            size: Duration::from_secs(20),
            depth: Duration::from_secs(20),
            search: Duration::from_secs(20),
            remove: Duration::from_secs(20),
        });
    }

    #[test]
    fn calc_average_time_counts() {
        let mut test_vec = Vec::<TimeCounts>::new();

        test_vec.push(TimeCounts {
            insert: Duration::from_secs(5) ,
            size: Duration::from_secs(5),
            depth: Duration::from_secs(5),
            search: Duration::from_secs(5),
            remove: Duration::from_secs(5),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(4) ,
            size: Duration::from_secs(4),
            depth: Duration::from_secs(4),
            search: Duration::from_secs(4),
            remove: Duration::from_secs(4),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(3) ,
            size: Duration::from_secs(3),
            depth: Duration::from_secs(3),
            search: Duration::from_secs(3),
            remove: Duration::from_secs(3),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(7) ,
            size: Duration::from_secs(7),
            depth: Duration::from_secs(7),
            search: Duration::from_secs(7),
            remove: Duration::from_secs(7),
        });
        test_vec.push(TimeCounts {
            insert: Duration::from_secs(1) ,
            size: Duration::from_secs(1),
            depth: Duration::from_secs(1),
            search: Duration::from_secs(1),
            remove: Duration::from_secs(1),
        });
 
        assert_eq!(super::calc_average_time_counts(&test_vec), TimeCounts{
            insert: Duration::from_secs(4),
            size: Duration::from_secs(4),
            depth: Duration::from_secs(4),
            search: Duration::from_secs(4),
            remove: Duration::from_secs(4),
        });
    }
}
