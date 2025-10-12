use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{error::Error, fs};

// ----------------------------
// Data Structures
// ----------------------------

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    id: u32,
    amount: u32,
    sender: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    id: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    prev_hash: String,
    #[serde(skip)]
    hash: String,
}

// ----------------------------
// Implementations
// ----------------------------

impl Block {
    /// Compute SHA-256 hash of the serialized block
    fn compute_hash(&self) -> String {
        let serialized = serde_json::to_string(self).expect("Serialization failed");
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }

    /// Constructor for creating a new block with computed hash
    fn new(id: u32, timestamp: u64, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        let mut block = Block {
            id,
            timestamp,
            transactions,
            prev_hash,
            hash: String::new(),
        };
        block.hash = block.compute_hash();
        block
    }
}

// ----------------------------
// Helper Functions
// ----------------------------

/// Verify the integrity of a blockchain
fn verify_chain(chain: &[Block]) -> bool {
    for i in 1..chain.len() {
        if chain[i].prev_hash != chain[i - 1].hash {
            return false;
        }
    }
    true
}

// ----------------------------
// Main Function
// ----------------------------

fn main() -> Result<(), Box<dyn Error>> {
    // ----------------------------
    // Create Genesis Block
    // ----------------------------
    let genesis = Block::new(
        0,
        1631234566,
        vec![Transaction {
            id: 1,
            amount: 50,
            sender: "Genesis".to_string(),
        }],
        "0".to_string(), // Genesis block has no prev_hash
    );

    // ----------------------------
    // Create Block 1
    // ----------------------------
    let block1 = Block::new(
        1,
        1631234567,
        vec![
            Transaction {
                id: 2,
                amount: 100,
                sender: "Alice".to_string(),
            },
            Transaction {
                id: 3,
                amount: 200,
                sender: "Bob".to_string(),
            },
        ],
        genesis.hash.clone(),
    );

    // ----------------------------
    // Serialize blockchain
    // ----------------------------
    let chain = vec![genesis, block1];
    let serialized_chain = serde_json::to_string_pretty(&chain)?;
    println!("Serialized blockchain:\n{}", serialized_chain);

    // ----------------------------
    // Deserialize blockchain
    // ----------------------------
    let deserialized_chain: Vec<Block> = serde_json::from_str(&serialized_chain)?;
    println!("Deserialized blockchain: {:?}", deserialized_chain);

    // ----------------------------
    // Verify blockchain integrity
    // ----------------------------
    if verify_chain(&deserialized_chain) {
        println!("Blockchain integrity verified ✅");
    } else {
        println!("Blockchain integrity failed ❌");
    }

    // ----------------------------
    // Save a block to a file
    // ----------------------------
    let block = &deserialized_chain[1];
    let serialized_block = serde_json::to_string_pretty(block)?;
    fs::write("block.json", &serialized_block)?;
    let file_content = fs::read_to_string("block.json")?;
    let block_from_file: Block = serde_json::from_str(&file_content)?;
    println!("Deserialized block from file: {:?}", block_from_file);

    // ----------------------------
    // Handle invalid JSON
    // ----------------------------
    let invalid_json = r#"{"id":1,"timestamp":1631234567}"#; // Missing transactions
    let result = serde_json::from_str::<Block>(invalid_json);
    println!("Invalid JSON result: {:?}", result); // Expect error

    Ok(())
}
