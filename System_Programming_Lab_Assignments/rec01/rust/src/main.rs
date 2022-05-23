extern crate hex;

/*
    Name: Tyler Adam Martinez
    EUID: tam0301
    Section: 206
*/

fn main() {
    println!("Enter an ASCII character : ");
    
    let mut string_user_input: String = String::new();
    std::io::stdin().read_line(&mut string_user_input).unwrap();

    let str_user_input: &str = &string_user_input[0..1];
    println!("The ASCII value of {} is:", str_user_input);


    for char_user_input in str_user_input.chars() {
        println!("dec -- {}", char_user_input as u32);
    }
    println!("hex -- {}", hex::encode(str_user_input));
    for char_user_input in str_user_input.clone().as_bytes() {
        println!("bin -- {:b} ", char_user_input);
    }
}
