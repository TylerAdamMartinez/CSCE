use homework3::Dictionary;

fn main() {
    let ten_thousand = 10000;
    let mut open_hashed_dictionary: Dictionary =
        Dictionary::new().open_hasing().build(ten_thousand);
    open_hashed_dictionary.insert(String::from("Janet"), String::from("Jackson"));
    open_hashed_dictionary.insert(String::from("Micheal"), String::from("Jackson"));
    println!(
        "key: Janet, and value: {}",
        open_hashed_dictionary.find_item(&String::from("Janet"))
    );
    println!(
        "key: Micheal, and value: {}",
        open_hashed_dictionary.find_item(&String::from("Micheal"))
    );

    open_hashed_dictionary.remove(&String::from("Micheal"));
    println!(
        "key: Micheal, and value: {:#?}",
        open_hashed_dictionary.find_item(&String::from("Micheal"))
    );
}
