use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Dictionary {
    keys_list: Vec<String>,
    table: Vec<Option<String>>,
    open_hasing: bool,
    number_of_elements: u64,
}

impl Dictionary {
    fn new() -> DictionaryBuilder {
        DictionaryBuilder {
            keys_list: Vec::<String>::new(),
            table: Vec::<Option<String>>::new(),
            open_hasing: None,
        }
    }
}

struct DictionaryBuilder {
    keys_list: Vec<String>,
    table: Vec<Option<String>>,
    open_hasing: Option<bool>,
}

impl DictionaryBuilder {
    fn open_hasing(&mut self) -> &mut Self {
        self.open_hasing = Some(true);
        self
    }

    fn closed_hasing(&mut self) -> &mut Self {
        self.open_hasing = Some(false);
        self
    }

    fn build(&mut self, number_of_elements: u64) -> Dictionary {
        // create an array of size number_of_elements in memory for the hash table
        for _ in 0..number_of_elements {
            self.table.push(None);
        }

        Dictionary {
            keys_list: self.keys_list.clone(),
            table: self.table.clone(),
            // default will yeild false therefore it will be closed hashing if not specified
            open_hasing: self.open_hasing.unwrap_or_default(),
            number_of_elements,
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Dictionary {
    fn calculate_index(&self, hash_value: u64) -> u16 {
        let max_array_elements = self.number_of_elements - 1;
        (hash_value % max_array_elements).try_into().unwrap()
    }

    fn insert(&mut self, key: String, value: String) -> bool {
        self.keys_list.push(key.clone());
        let index: usize = self.calculate_index(calculate_hash(&key)) as usize;
        self.table[index] = Some(value);
        true
    }

    fn find_item(&self, key: String) -> String {
        let index: usize = self.calculate_index(calculate_hash(&key)) as usize;
        self.table[index].clone().unwrap()
    }
}

fn main() {
    let ten_thousand = 10000;
    let closed_hashed_dictionary = Dictionary::new().closed_hasing().build(ten_thousand);
    let mut open_hashed_dictionary = Dictionary::new().open_hasing().build(ten_thousand);
    println!("{}", open_hashed_dictionary.calculate_index(std::u64::MAX));
    println!(
        "{}",
        closed_hashed_dictionary.calculate_index(std::u64::MIN)
    );

    open_hashed_dictionary.insert(String::from("Janet"), String::from("Jackson"));
    println!(
        "key: Janet, and value: {}",
        open_hashed_dictionary.find_item(String::from("Janet"))
    );
}
