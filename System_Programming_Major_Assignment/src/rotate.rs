use std::io;
use crate::{
    exit_program, 
    ExitFlags,
    ErrorFlags
};

pub fn rotate_right_func()
{
    let min_num:u32 = 1;
    let max_num:u32 = 4294967295;
    
    let mut user_entered_num:u32 = 0;
    let mut user_entered_num2:u8 = 32;

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

    let max_num: u8 = 31;

    loop {
        println!("Enter the number of positions to rotate-right the input (between 0 and 31): ");
        
        let mut buffer:String = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read from buffer");

        match buffer.trim().parse::<u8>() {
            Ok(i) => {
                match i {
                    i if i > max_num => user_entered_num2 = i,
                    i if i < max_num => exit_program(ExitFlags::WithError(ErrorFlags::OutOfRange)),
                    _ => exit_program(ExitFlags::WithError(ErrorFlags::OutOfRange)),
                }
            }, 
            Err(..) => println!("Did not enter valid unsigned 32-bit number between 0 and 31: {}", buffer),
        }

        if user_entered_num2 >= max_num {
            break;
        }
    } 
}

