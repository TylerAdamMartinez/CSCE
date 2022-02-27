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
    let time_counts_sum = calc_time_counts_sum(&time_counts_vec); 

    let insert_average = time_counts_sum.insert / time_counts_vec.len().try_into().unwrap();
    let size_average = time_counts_sum.size / time_counts_vec.len().try_into().unwrap();
    let depth_average = time_counts_sum.depth / time_counts_vec.len().try_into().unwrap();
    let search_average = time_counts_sum.search / time_counts_vec.len().try_into().unwrap();
    let remove_average = time_counts_sum.remove / time_counts_vec.len().try_into().unwrap();

    // remove_average = remove_average / time_counts_vec.len().try_into().unwrap();

    TimeCounts {
        insert: insert_average,
        size: size_average,
        depth: depth_average,
        search: search_average,
        remove: remove_average,
    }
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
