use std::{sync::mpsc, thread::{self, spawn}};
//mpsc -> multiple prodiucer single consumer 

fn main(){

//find the sum for 1 to 1000
let (tx ,rx)= mpsc::channel();
let (tx1 ,rx1)= mpsc::channel();

 thread::spawn( move || {
    let mut sum1 = 0;
for i in 1..=200{
 sum1 = sum1+i;
}
tx.send(sum1).unwrap();
 });

 thread::spawn(move  || {
    let mut sum2 = 0;
    for i in 201..=400{
  sum2 = sum2 + i;
    }
    tx1.send(sum2).unwrap();
 });
// let  mut total_sum = 0;
// while let Ok(message) = rx.recv() {
//      total_sum = total_sum + message;
// }



let recieved1 = rx1.recv().unwrap();
println!("Total sum form 1 to 200: {}",recieved1);
let recieved2 = rx.recv().unwrap();
println!("Total sum form 200 to 401: {}",recieved2);
 let total_sum = recieved1 + recieved2;
println!("Total sum form 1 to 400: {}",total_sum);
}