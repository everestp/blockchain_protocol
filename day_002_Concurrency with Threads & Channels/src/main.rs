



pub mod messge_passing;

use std::{thread, time::Duration};

fn main() {

    

    //moving data into the threads
    let v = vec![1,2,3,4];
    let handle = thread::spawn( move|| {
        println!("Here is the vector :{:?}",v);
    });
   handle.join();

     let handle =  thread::spawn( || {
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

// moving data into Threads
