use std::io;

pub fn parity_func()
{
    let min_num:u32 = 1;
    let max_num:u32 = 4294967295;

    let mut user_entered_num:u32 = 0;
    loop {
        println!("Enter a 32-bit number (>= 1 and <= 4294967295, inclusively): ");
        
        let mut buffer:String = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read from buffer");

        match buffer.trim().parse::<u32>() {
            Ok(i) => user_entered_num = i, 
            Err(..) => println!("Did not enter valid unsigned 32-bit number: {}", buffer),
        }
        
        if user_entered_num >= min_num && user_entered_num <= max_num {
            break;
        }
    }

    let mut rightmost_bit = user_entered_num ^ (user_entered_num >> 1);
    rightmost_bit = rightmost_bit ^ (rightmost_bit >> 2);
    rightmost_bit = rightmost_bit ^ (rightmost_bit >> 4);
    rightmost_bit = rightmost_bit ^ (rightmost_bit >> 8);
    rightmost_bit = rightmost_bit ^ (rightmost_bit >> 16);

    if (rightmost_bit & 1) == 1
    {
        println!("Parity of {} is 1", user_entered_num);
    }
    else
    {
        println!("Parity of {} is 0", user_entered_num);
    }
}
