use std::io;
use std::process::exit;
mod parity;
mod clz;
mod endian;
mod rotate;

/*
    Name: Tyler Adam Martinez
    EUID: tam0301
*/

#[derive(PartialEq)]
enum ErrorFlags
{
    InvalidInput,
    OutOfRange,
}

#[derive(PartialEq)]
enum ExitFlags
{
    Safely,
    WithError(ErrorFlags),
}

fn main()
{
    #[derive(PartialEq)]
    enum MenuOptions
    {
        LeadingZeros,
        EndingSwap,
        RotateRight,
        Parity,
        Exit,
        Invalid,
    }

    let mut menu_option: MenuOptions = MenuOptions::Invalid;

    while menu_option != MenuOptions::Exit
    {
        let mut buffer:String = String::new();

        display_menu_options();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read from buffer");

        match buffer.trim().parse::<u8>() {
            Ok(i) => {
                match i
                {
                    1 => menu_option = MenuOptions::LeadingZeros,
                    2 => menu_option = MenuOptions::EndingSwap,
                    3 => menu_option = MenuOptions::RotateRight,
                    4 => menu_option = MenuOptions::Parity,
                    5 => menu_option = MenuOptions::Exit,
                    _ => menu_option = MenuOptions::Invalid,
                }
            }, 
            Err(..) => println!("Did not enter a vaild menu option: {}", buffer),
        }

        match menu_option
        {
            MenuOptions::LeadingZeros => clz::leading_zeros_func(),
            MenuOptions::EndingSwap => endian::ending_swap_func(),
            MenuOptions::RotateRight => rotate::rotate_right_func(),
            MenuOptions::Parity => parity::parity_func(),
            MenuOptions::Exit => exit_program(ExitFlags::Safely),
            MenuOptions::Invalid => exit_program(ExitFlags::WithError(ErrorFlags::InvalidInput)),
        }
    }
}

fn display_menu_options()
{
    println!("Enter the menu option for the operation to preform:");
    println!("(1) Count Leading Zeroes");
    println!("(2) Endian Swap");
    println!("(3) Rotate-right");
    println!("(4) Parity");
    println!("(5) EXIT");
    println!("--> ");
}

fn exit_program(flag: ExitFlags)
{
    match flag
    {
        ExitFlags::Safely => {
            println!("Program terminating. Goodbye...");
            println!("Successful program terminated with zero errors.");
            exit(0);
        }
        ExitFlags::WithError(ErrorFlags::InvalidInput) => {
            println!("Program terminating. Goodbye...");
            println!("Unsuccessful program terminated with error code one.");
            println!("Invalid menu option");
            exit(0);
        }
        ExitFlags::WithError(ErrorFlags::OutOfRange) => {
            println!("Program terminating. Goodbye...");
            println!("Unsuccessful program terminated with error code two.");
            println!("Out of range!");
            exit(0);       
        }
    }

}
