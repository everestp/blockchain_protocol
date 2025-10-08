



use std::{thread, time::Duration};

fn main() {

     thread::spawn(|| {
        for   i in 1..5 {
            println!("Thread :{}",i);
            thread::sleep(Duration::from_millis(1500));
        }
    });
    for i in 1..3{
        println!("Main :{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    
     handle.join().unwrap(); // wait thread to  finished
    println!("Hello, world!");
}

