#![feature(linked_list_remove)]
use std::collections::{hash_map::DefaultHasher, LinkedList};
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct KeyValuePair {
    key: String,
    value: String,
}

#[derive(Clone)]
enum DictionaryHashingType {
    Open,
    Closed,
}

impl Default for DictionaryHashingType {
    fn default() -> Self {
        DictionaryHashingType::Open
    }
}

pub struct Dictionary {
    keys_list: Vec<String>,
    table: Vec<LinkedList<KeyValuePair>>,
    hashing_type: DictionaryHashingType,
    number_of_elements: u64,
}

impl Dictionary {
    pub fn new() -> DictionaryBuilder {
        DictionaryBuilder {
            keys_list: Vec::<String>::new(),
            table: Vec::<LinkedList<KeyValuePair>>::new(),
            hashing_type: None,
        }
    }
}

pub struct DictionaryBuilder {
    keys_list: Vec<String>,
    table: Vec<LinkedList<KeyValuePair>>,
    hashing_type: Option<DictionaryHashingType>,
}

impl DictionaryBuilder {
    pub fn open_hashing(&mut self) -> &mut Self {
        self.hashing_type = Some(DictionaryHashingType::Open);
        self
    }

    pub fn closed_hashing(&mut self) -> &mut Self {
        self.hashing_type = Some(DictionaryHashingType::Closed);
        self
    }

    pub fn build(&mut self, number_of_elements: u64) -> Dictionary {
        // create an array of size number_of_elements in memory for the hash table
        for _ in 0..number_of_elements {
            self.table.push(LinkedList::<KeyValuePair>::new());
        }

        Dictionary {
            keys_list: self.keys_list.clone(),
            table: self.table.clone(),
            // If self.hashing_type is still the None varient by time the build function is called,
            // then hashing_type will default to DictionaryHashingType::Open
            hashing_type: self.hashing_type.clone().unwrap_or_default(),
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
        // If key is already taken then insert() function returns false
        // else adds key into keys_list and inserts the KeyValuePair in the Dictionary
        for key_item in self.keys_list.iter() {
            if &key == key_item {
                return false;
            }
        }
        self.keys_list.push(key.clone());

        let index: usize = self.calculate_index(calculate_hash(&key));
        match self.hashing_type {
            DictionaryHashingType::Open => {
                self.table[index].push_back(KeyValuePair { key, value });
            }
            DictionaryHashingType::Closed => match self.table[index].back() {
                None => self.table[index].push_back(KeyValuePair { key, value }),
                Some(..) => {
                    let mut i: usize = 1;
                    loop {
                        if (index + i) >= (self.number_of_elements as usize - 1) {
                            return false;
                        }

                        match self.table[index + i].back() {
                            Some(..) => i += 1,
                            None => {
                                self.table[index + i].push_back(KeyValuePair { key, value });
                                break;
                            }
                        }
                    }
                }
            },
        }
        true
    }

    pub fn find_item(&self, key: &String) -> Option<String> {
        let index: usize = self.calculate_index(calculate_hash(&key));
        match self.hashing_type {
            DictionaryHashingType::Open => {
                for pair in self.table[index].iter() {
                    if &pair.key == key {
                        return Some(pair.value.clone());
                    }
                }
            }
            DictionaryHashingType::Closed => match self.table[index].back() {
                None => return None,
                Some(pair) => {
                    let mut i: usize = 1;
                    loop {
                        if (index + i) >= (self.number_of_elements as usize - 1) {
                            return None;
                        } else {
                            if &pair.key == key {
                                return Some(pair.value.clone());
                            }
                        }

                        match self.table[index + i].back() {
                            Some(pair) => {
                                if &pair.key == key {
                                    return Some(pair.value.clone());
                                }
                            }
                            None => return None,
                        }
                        i += 1;
                    }
                }
            },
        }
        None
    }

    pub fn remove(&mut self, key: &String) -> bool {
        let index: usize = self.calculate_index(calculate_hash(&key));
        match self.hashing_type {
            DictionaryHashingType::Open => {
                let mut i: usize = 0;
                for pair in self.table[index].iter() {
                    if &pair.key == key {
                        self.table[index].remove(i);
                        return true;
                    }
                    i += 1;
                }
            }
            DictionaryHashingType::Closed => match self.table[index].back() {
                None => return false,
                Some(pair) => {
                    let mut i: usize = 1;
                    loop {
                        if (index + i) >= (self.number_of_elements as usize - 1) {
                            return false;
                        } else {
                            if &pair.key == key {
                                self.table[index + i].remove(i);
                                return true;
                            }
                        }

                        match self.table[index + i].back() {
                            Some(pair) => {
                                if &pair.key == key {
                                    self.table[index + i].remove(i);
                                    return true;
                                }
                            }
                            None => return false,
                        }
                        i += 1;
                    }
                }
            },
        }
        false
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

#[cfg(test)]
mod dictionary_open_hashing_tests {
    use super::*;

    #[test]
    fn insert() {
        let mut new_dictionary: Dictionary = Dictionary::new().build(5);
        let new_key_outcome = new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let old_key_outcome = new_dictionary.insert(String::from("Uchiha"), String::from("Sasuke"));

        assert_eq!(new_key_outcome, true);
        assert_eq!(old_key_outcome, false);
    }

    #[test]
    fn find_item_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let find_item_outcome = new_dictionary.find_item(&String::from("Uchiha")).unwrap();

        assert_eq!(find_item_outcome, String::from("Itachi"));
    }

    #[test]
    fn find_item_not_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let find_item_outcome = new_dictionary.find_item(&String::from("Uzumaki"));

        assert_eq!(find_item_outcome, None);
    }

    #[test]
    fn remove_item_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let remove_item_outcome = new_dictionary.remove(&String::from("Uchiha"));

        assert_eq!(remove_item_outcome, true);
    }

    #[test]
    fn remove_item_not_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let remove_item_outcome = new_dictionary.remove(&String::from("Uzumaki"));

        assert_eq!(remove_item_outcome, false);
    }
}

#[cfg(test)]
mod dictionary_closed_hashing_tests {
    use super::*;

    #[test]
    fn insert() {
        let mut new_dictionary: Dictionary = Dictionary::new().closed_hashing().build(5);
        let new_key_outcome = new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let old_key_outcome = new_dictionary.insert(String::from("Uchiha"), String::from("Sasuke"));

        assert_eq!(new_key_outcome, true);
        assert_eq!(old_key_outcome, false);
    }

    #[test]
    fn find_item_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().closed_hashing().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let find_item_outcome = new_dictionary.find_item(&String::from("Uchiha")).unwrap();

        assert_eq!(find_item_outcome, String::from("Itachi"));
    }

    #[test]
    fn find_item_not_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().closed_hashing().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let find_item_outcome = new_dictionary.find_item(&String::from("Uzumaki"));

        assert_eq!(find_item_outcome, None);
    }

    #[test]
    fn remove_item_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().closed_hashing().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let remove_item_outcome = new_dictionary.remove(&String::from("Uchiha"));

        assert_eq!(remove_item_outcome, true);
    }

    #[test]
    fn remove_item_not_found() {
        let mut new_dictionary: Dictionary = Dictionary::new().closed_hashing().build(5);
        new_dictionary.insert(String::from("Uchiha"), String::from("Itachi"));
        let remove_item_outcome = new_dictionary.remove(&String::from("Uzumaki"));

        assert_eq!(remove_item_outcome, false);
    }
}
