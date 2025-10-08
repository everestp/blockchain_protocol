use std::{sync::mpsc::{self, Sender}, thread};

// mpsc -> multiple producer single consumer

fn main() {

 #[derive(Debug)]
 enum TxResult {
    Valid(u32),
    Invalid(String),

 }

 fn validate_tx_in_thread(tx_id:u32 ,sender: Sender<TxResult>){
    // simulkate the validation 
    if tx_id % 2 ==0{
        sender.send(TxResult::Valid(tx_id)).unwrap();
    }else {
         sender.send(TxResult::Invalid("Invalid signature".to_string())).unwrap();
    }

 }





    // -------------------- First Part --------------------
    // Find the sum using multiple threads
    let (tx, rx) = mpsc::channel();

    for i in 0..=2 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans = 0;
            for j in 0..10 {
                ans += i * 10 + j;
                println!("Thread {i}: processing {j}");
            }
            producer.send(ans).unwrap();
        });
    }

    drop(tx); // important: close main sender

    let mut total = 0;
    for val in rx {
        total += val;
    }

    println!("Total Sum from first part = {}", total);

    // -------------------- Second Part --------------------
    // Find the sum for 1 to 400 using two threads

    let (tx, rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();

    thread::spawn(move || {
        let mut sum1 = 0;
        for i in 1..=200 {
            sum1 += i;
        }
        tx.send(sum1).unwrap();
    });

    thread::spawn(move || {
        let mut sum2 = 0;
        for i in 201..=400 {
            sum2 += i;
        }
        tx1.send(sum2).unwrap();
    });

    let received1 = rx.recv().unwrap();
    println!("Sum from 1 to 200: {}", received1);

    let received2 = rx1.recv().unwrap();
    println!("Sum from 201 to 400: {}", received2);

    let total_sum = received1 + received2;
    println!("Total Sum from 1 to 400: {}", total_sum);
}
