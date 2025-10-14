use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::{error::Error, sync::mpsc};
use std::thread;

/// SafeNumber wraps a raw pointer to an i32 value.
struct SafeNumber {
    ptr: *mut i32,
}

impl SafeNumber {
    fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        SafeNumber { ptr }
    }

    fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }

    fn set(&mut self, value: i32) {
        unsafe { *self.ptr = value }
    }
}

impl Drop for SafeNumber {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.ptr); }
    }
}

/// Represents a transaction with id, amount, and sender.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    id: u32,
    amount: u64,
    sender: String,
}

/// Mempool holds transactions in a raw pointer vector.
struct Mempool {
    ptr: *mut Vec<Transaction>,
    capacity: usize,
}

impl Mempool {
    /// Create a new empty mempool with reserved capacity.
    fn new(capacity: usize) -> Self {
        let vec = Vec::with_capacity(capacity);
        let boxed = Box::new(vec);
        let ptr = Box::into_raw(boxed);
        Mempool { ptr, capacity }
    }

    /// Add a transaction if capacity allows.
    fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        unsafe {
            if (*self.ptr).len() >= self.capacity {
                return Err("Mempool full".to_string());
            }
            (*self.ptr).push(tx);
            Ok(())
        }
    }

    /// Remove transaction by id.
    fn remove_transaction(&mut self, id: u32) -> Result<(), String> {
        unsafe {
            if let Some(pos) = (*self.ptr).iter().position(|tx| tx.id == id) {
                (*self.ptr).remove(pos);
                Ok(())
            } else {
                Err("Transaction not found".to_string())
            }
        }
    }

    /// Serialize transactions with amount < 1000.
    fn serialize_valid(&self) -> Result<String, serde_json::Error> {
        unsafe {
            let valid: Vec<Transaction> =
                (*self.ptr).iter().filter(|tx| tx.amount < 1000).cloned().collect();
            serde_json::to_string_pretty(&valid)
        }
    }

    /// Get a transaction by index.
    fn get_transaction(&self, index: usize) -> Option<&Transaction> {
        unsafe { (&*self.ptr).get(index) }
    }

    /// Compute SHA-256 hash of the mempool.
    fn compute_hash(&self) -> String {
        unsafe {
            let serialized = serde_json::to_string(&*self.ptr).expect("Serialization failed");
            let mut hasher = Sha256::new();
            hasher.update(serialized);
            format!("{:x}", hasher.finalize())
        }
    }
}

impl Drop for Mempool {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.ptr); }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // ------------------ Channel for concurrency ------------------
    let (tx_channel, rx_channel) = mpsc::channel::<Transaction>();

    // Spawn a thread to receive and print validated transactions
    thread::spawn(move || {
        for msg in rx_channel {
            println!("Validated transaction received: {:?}", msg);
        }
    });

    // ------------------ Mempool demonstration ------------------
    let mut mempool = Mempool::new(3);

    let t1 = Transaction { id: 1, amount: 100, sender: "Alice".into() };
    let t2 = Transaction { id: 2, amount: 200, sender: "Bob".into() };

    mempool.add_transaction(t1.clone())?;
    mempool.add_transaction(t2.clone())?;

    if let Some(tx) = mempool.get_transaction(0) {
        println!("Transaction 0: {:?}", tx);
    }

    mempool.remove_transaction(1)?;
    println!("After removal, transaction 0: {:?}", mempool.get_transaction(0));

    println!("Valid transactions serialized:\n{}", mempool.serialize_valid()?);

    // Send a transaction to the validation thread
    if let Some(tx) = mempool.get_transaction(0) {
        tx_channel.send(tx.clone())?;
    }

    println!("Mempool hash: {}", mempool.compute_hash());

    let serialized = unsafe { serde_json::to_string_pretty(&*mempool.ptr)? };
    println!("Serialized mempool:\n{}", serialized);

    let deserialized: Vec<Transaction> = serde_json::from_str(&serialized)?;
    println!("Deserialized mempool: {:?}", deserialized);

    // ------------------ SafeNumber demonstration ------------------
    let mut num = SafeNumber::new(42);
    println!("SafeNumber initial: {}", num.get());
    num.set(100);
    println!("SafeNumber updated: {}", num.get());

    // ------------------ Raw pointer demo ------------------
    let mut data = 42;
    let ptr: *mut i32 = &mut data;
    let ptr2: *const i32 = &data;

    unsafe {
        *ptr = 3;
        println!("Read via ptr2: {}", *ptr2);
    }
    println!("Data after raw pointer update: {}", data);

    Ok(())
}
