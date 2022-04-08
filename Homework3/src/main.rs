use homework3::Dictionary;

fn main() {
    let ten_thousand = 10000;
    let mut open_hashed_dictionary: Dictionary =
        Dictionary::new().open_hashing().build(ten_thousand);
    open_hashed_dictionary.insert(String::from("Boruto"), String::from("Uzumaki"));
    open_hashed_dictionary.insert(String::from("Naruto"), String::from("Uzumaki"));
    open_hashed_dictionary.insert(String::from("Hinata"), String::from("Uzumaki"));

    println!(
        "Boruto {} is the son of Naruto {} and Hinata {}",
        open_hashed_dictionary
            .find_item(&String::from("Boruto"))
            .unwrap(),
        open_hashed_dictionary
            .find_item(&String::from("Naruto"))
            .unwrap(),
        open_hashed_dictionary
            .find_item(&String::from("Hinata"))
            .unwrap()
    );

    // since you cannot insert the same key twice the value for Hinata is still Uzumaki
    open_hashed_dictionary.insert(String::from("Hinata"), String::from("Hyūga"));
    println!(
        "Since Hinata last name has already been entered as {}, we cannot change to her last name back to her maiden name of Hyūga",
        open_hashed_dictionary
            .find_item(&String::from("Hinata"))
            .unwrap()
    );

    open_hashed_dictionary.remove(&String::from("Boruto"));
    // after remove Boruto the value will yield None
    println!(
        "Value after KeyValuePair has been removed is {:?}",
        open_hashed_dictionary.find_item(&String::from("Boruto"))
    );

    let mut closed_hashed_dictionary: Dictionary =
        Dictionary::new().closed_hashing().build(ten_thousand);
    closed_hashed_dictionary.insert(String::from("Fugaku"), String::from("Uchiha"));
    closed_hashed_dictionary.insert(String::from("Itachi"), String::from("Uchiha"));
    closed_hashed_dictionary.insert(String::from("Sasuke"), String::from("Uchiha"));

    println!(
        "Fugaku {} is the father of Itachi {} and Sasuke {}",
        closed_hashed_dictionary
            .find_item(&String::from("Fugaku"))
            .unwrap(),
        closed_hashed_dictionary
            .find_item(&String::from("Itachi"))
            .unwrap(),
        closed_hashed_dictionary
            .find_item(&String::from("Sasuke"))
            .unwrap()
    );

    // since you cannot insert the same key twice the value for Itachi is still Akatsuki
    closed_hashed_dictionary.insert(String::from("Itachi"), String::from("Akatsuki"));
    println!(
        "Since Itachi last name has already been entered as {}, we cannot change to his last name Akatsuki",
        closed_hashed_dictionary
            .find_item(&String::from("Itachi"))
            .unwrap()
    );

    closed_hashed_dictionary.remove(&String::from("Fugaku"));
    // after remove Boruto the value will yield None
    println!(
        "Value after KeyValuePair has been removed is {:?}",
        closed_hashed_dictionary.find_item(&String::from("Fugaku"))
    );
}
