use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct KeyValuePair {
    key: String,
    value: String,
}

pub struct Dictionary {
    keys_list: Vec<String>,
    table: Vec<Option<KeyValuePair>>,
    open_hasing: bool,
    number_of_elements: u64,
}

impl Dictionary {
    pub fn new() -> DictionaryBuilder {
        DictionaryBuilder {
            keys_list: Vec::<String>::new(),
            table: Vec::<Option<KeyValuePair>>::new(),
            open_hasing: None,
        }
    }
}

pub struct DictionaryBuilder {
    keys_list: Vec<String>,
    table: Vec<Option<KeyValuePair>>,
    open_hasing: Option<bool>,
}

impl DictionaryBuilder {
    pub fn open_hasing(&mut self) -> &mut Self {
        self.open_hasing = Some(true);
        self
    }

    pub fn closed_hasing(&mut self) -> &mut Self {
        self.open_hasing = Some(false);
        self
    }

    pub fn build(&mut self, number_of_elements: u64) -> Dictionary {
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
    fn calculate_index(&self, hash_value: u64) -> usize {
        let max_array_elements = self.number_of_elements - 1;
        (hash_value % max_array_elements).try_into().unwrap()
    }

    pub fn insert(&mut self, key: String, value: String) -> bool {
        self.keys_list.push(key.clone());
        let index: usize = self.calculate_index(calculate_hash(&key));
        self.table[index] = Some(KeyValuePair { key, value });
        true
    }

    pub fn find_item(&self, key: &String) -> String {
        let index: usize = self.calculate_index(calculate_hash(&key));
        self.table[index].clone().unwrap().value
    }

    pub fn remove(&mut self, key: &String) -> bool {
        let index: usize = self.calculate_index(calculate_hash(&key));
        self.table[index] = None;
        true
    }
}

#[cfg(test)]
mod dictionary_tests {
    use super::*;

    #[test]
    fn good_hash() {
        let hash1: u64 = calculate_hash(&"Tyler".to_string());
        let hash2: u64 = calculate_hash(&"Tiler".to_string());

        assert!(hash1 != hash2);
        assert_eq!(hash1, 577972250856374879);
        assert_eq!(hash2, 17815629583754718023);
    }

    #[test]
    fn good_indexes() {
        let new_dictionary: Dictionary = Dictionary::new().build(100);
        let index1: usize = new_dictionary.calculate_index(calculate_hash(&"Tyler".to_string()));
        let index2: usize = new_dictionary.calculate_index(calculate_hash(&"Tiler".to_string()));

        assert!(index1 != index2);
        assert_eq!(index1, 65);
        assert_eq!(index2, 11);
    }
}
