use std::{thread, time::Duration};
fn main() {
    let th = thread::spawn(
        || {
            for i in 1..10 {
                println!("Thread-1 : Num = {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        }
    );
    // here the main thread
    for i in 1..7 {
        println!("Main-Thread: Num = {}", i);
        thread::sleep(Duration::from_millis(1))
    }
    // here we tell the main thread to wait for the th thread to finish before going on
    th.join().unwrap();
    println!("Done!");

}
