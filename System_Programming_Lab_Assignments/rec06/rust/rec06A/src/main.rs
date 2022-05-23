use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let thread_status = thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        println!("ps -e -o pid,ppid,stat,user,cmd | grep $USER");
    });

    match thread_status.join() {
        Ok(..) => println!("It work!"),
        Err(..) => println!("thread::spawn Error"),
    }

}
