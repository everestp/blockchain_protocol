use std::{
    fmt::Debug,
    sync::mpsc::{self, Sender},
    thread,
};

// mpsc -> multiple producer single consumer

#[derive(Debug)]
enum TxResult {
    Valid(u32),
    Invalid(String),
}

fn validate_tx_in_thread(tx_id: u32, sender: Sender<TxResult>) {
    // simulate the validation
    if tx_id % 2 == 0 {
        sender.send(TxResult::Valid(tx_id)).unwrap();
    } else {
        sender
            .send(TxResult::Invalid("Invalid signature".to_string()))
            .unwrap();
    }
}

#[derive(Debug)]
struct Tx {
    id: u32,
    amount: u64,
}

#[derive(Debug)]
enum GossipMsg {
    ValidTx(Tx),
    Error(String),
}

// validator function
fn validate_tx(tx: Tx) -> Result<Tx, String> {
    if tx.amount > 100 {
        Err("Amount too high".to_string())
    } else {
        Ok(tx)
    }
}

fn main() {
    // -------------------- Gossip Validation Simulation --------------------
    let (tx_channel, rx_channel) = mpsc::channel::<GossipMsg>();
    let mut handles = vec![];

    for node_id in 0..2 {
        let sender = tx_channel.clone();
        let handle = thread::spawn(move || {
            let tx = Tx {
                id: node_id,
                amount: (node_id as u64) * 50,
            };
            match validate_tx(tx) {
                Ok(valid_tx) => sender.send(GossipMsg::ValidTx(valid_tx)).unwrap(),
                Err(e) => sender
                    .send(GossipMsg::Error(format!("Node {} error: {}", node_id, e)))
                    .unwrap(),
            }
        });
        handles.push(handle);
    }

    // wait for threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx_channel); // drop sender to close receiver loop

    println!("\nCentral node gossip results:");
    for msg in rx_channel {
        println!("{:?}", msg);
    }

    // -------------------- Part 1: Multi-threaded Sum --------------------
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

    let total: i32 = rx.iter().sum();
    println!("\nTotal Sum from first part = {}", total);

    // -------------------- Part 2: Range Sum Example --------------------
    let (tx_a, rx_a) = mpsc::channel();
    let (tx_b, rx_b) = mpsc::channel();

    thread::spawn(move || {
        let sum1: u64 = (1..=200).sum();
        tx_a.send(sum1).unwrap();
    });

    thread::spawn(move || {
        let sum2: u64 = (201..=400).sum();
        tx_b.send(sum2).unwrap();
    });

    let received1 = rx_a.recv().unwrap();
    let received2 = rx_b.recv().unwrap();
    let total_sum = received1 + received2;

    println!("\nSum from 1 to 200: {}", received1);
    println!("Sum from 201 to 400: {}", received2);
    println!("Total Sum from 1 to 400: {}", total_sum);
}
