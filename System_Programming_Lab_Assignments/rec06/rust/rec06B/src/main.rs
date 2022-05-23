use std::thread;
use std::time::Duration;

fn main() {
    let child_thread = thread::spawn(|| {
        println!("child: started");
        println!("child: ... ");
        thread::sleep(Duration::from_secs(20));
    });

    println!("parent: started");
    println!("parent: terminating..");

    match child_thread.join() {
        Ok(_) => println!("The child has awaken!"),
        Err(e) => println!("child failed\n{:#?}", e),
    }

    println!("child: terminating..");
}
