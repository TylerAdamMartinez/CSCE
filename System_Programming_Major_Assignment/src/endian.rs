use std::io;

pub fn ending_swap_func()
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
}
