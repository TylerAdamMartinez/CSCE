use std::io;

/*
    Name: Tyler Adam Martinez
    EUID: tam0301
    Section: 206
*/

fn main() {
    let mut int_ptr: Vec<i32> = Vec::new();
    let mut user_entered_num: String =  String::new();
    
    println!("Enter first integer: ");
    io::stdin()
        .read_line(&mut user_entered_num)
        .expect("Failed to read in integer from terminal");

    match user_entered_num.trim().parse::<i32>() {
        Ok(i) => int_ptr.push(i),
        Err(..) => println!("This was not an integer: {}", user_entered_num),
    };
    
    let mut user_entered_num2: String =  String::new();
    println!("Enter second integer: ");
    io::stdin()
        .read_line(&mut user_entered_num2)
        .expect("Failed to read in integer from terminal");

    match user_entered_num2.trim().parse::<i32>() {
        Ok(i) => int_ptr.push(i),
        Err(..) => println!("This was not an integer: {}", user_entered_num2),
    };

    println!("Original values: 1st = {} 2nd = {}", int_ptr[0], int_ptr[1]);

    int_ptr[0] = int_ptr[0] ^ int_ptr[1];
    int_ptr[1] = int_ptr[1] ^ int_ptr[0];
    int_ptr[0] = int_ptr[0] ^ int_ptr[1];

    println!("Swapped values: 1st = {} 2nd = {}", int_ptr[0], int_ptr[1]);

}

