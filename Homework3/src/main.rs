use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Pair {
    key: String,
    value: String,
}

struct Dictionary {
    table: [Pair; 10000],
}

fn main() {
    let pair1 = Pair {
        key: "key1".to_string(),
        value: "Janet jackson".to_string(),
    };

    //assert!(calculate_hash(&person1) != calculate_hash(&person2));
    println!("{}", calculate_index(calculate_hash(&pair1.key)));
    println!("{}", calculate_index(std::u64::MAX));
}

fn calculate_index(hash_value: u64) -> u16 {
    (hash_value % 9999).try_into().unwrap()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
