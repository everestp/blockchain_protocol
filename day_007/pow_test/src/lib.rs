// --------------------------------------------
// Simple Blockchain Mining Example in Rust
// --------------------------------------------
//
// This example demonstrates a simplified proof-of-work (PoW) mining process.
// It defines a `Block` structure and a `mine_block` function that repeatedly
// hashes the block’s data with different nonce values until it finds a hash
// that starts with a number of leading zeros equal to the given difficulty.
//
// The difficulty controls how hard it is to mine a block: higher difficulty
// means more leading zeros required in the hash.
//
// --------------------------------------------

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

// --------------------------------------------
// Block Structure
// --------------------------------------------
// `Serialize` and `Deserialize` let us easily convert the struct to JSON.
// `Clone` is used for copying during mining.
// `Debug` is useful for printing during debugging.
// --------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u32,       // Unique identifier for the block
    pub nonce: u64,    // Nonce value found through mining
    pub data: String,  // Payload data (e.g., transactions)
}

// --------------------------------------------
// Mining Function
// --------------------------------------------
// Attempts to find a nonce that produces a SHA-256 hash starting
// with a specific number of leading zeros equal to `difficulty`.
//
// - Returns `Some(nonce)` if successful
// - Returns `None` if the difficulty is invalid (>64)
// --------------------------------------------
pub fn mine_block(block: &Block, difficulty: usize) -> Option<u64> {
    // Prevent excessive difficulty that could freeze or overflow
    if difficulty > 64 {
        return None;
    }

    // Build the target prefix of leading zeros (e.g., "00" for difficulty 2)
    let target = "0".repeat(difficulty);

    // Try every possible nonce value (0..=u64::MAX)
    for nonce in 0..=u64::MAX {
        // Clone the block so we can modify the nonce without changing the original
        let mut test_block = block.clone();
        test_block.nonce = nonce;

        // Compute the SHA-256 hash of the serialized block
        let hash = compute_hash(&test_block);
  println!("{}",hash);
        // Check if the hash meets the difficulty criteria
        if hash.starts_with(&target) {
            println!("✅ Block mined! Nonce: {nonce}, Hash: {hash}");
            return Some(nonce);
        }
    }

    // If the loop completes (extremely unlikely), return None
    None
}

// --------------------------------------------
// Hashing Function
// --------------------------------------------
// Converts the `Block` struct into a JSON string, then computes
// a SHA-256 hash of that string, returning the hash in hexadecimal.
// --------------------------------------------
fn compute_hash(block: &Block) -> String {
    // Convert the block into a JSON string
    let serialized = serde_json::to_string(block).expect("Serialization failed");

    // Initialize a SHA-256 hasher
    let mut hasher = Sha256::new();

    // Feed the serialized data into the hasher
    hasher.update(serialized);

    // Finalize the hash and convert the result into a hexadecimal string
    let result = hasher.finalize();
    format!("{:x}", result)
}

// --------------------------------------------
// Tests
// --------------------------------------------
// These unit tests validate that the mining logic works correctly.
// Run them with: `cargo test`
// --------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // Test that mining successfully finds a nonce that meets difficulty
    #[test]
    fn test_mine_block_valid() {
        let block = Block {
            id: 1,
            nonce: 0,
            data: String::from("test data"),
        };

        // Attempt to mine a block with difficulty 2
        let nonce = mine_block(&block, 2).expect("Mining failed");

        // Create a new block with the found nonce
        let mut mined_block = block.clone();
        mined_block.nonce = nonce;

        // Compute its hash and verify that it meets the difficulty
        let hash = compute_hash(&mined_block);
      
        assert!(
            hash.starts_with("00"),
            "Hash does not meet the required difficulty: {}",
            hash
        );
    }

    // Test that excessive difficulty returns None
    #[test]
    fn test_block_too_difficult() {
        let block = Block {
            id: 1,
            nonce: 0,
            data: String::from("test data"),
        };

        // Expect None for difficulty > 64
        assert_eq!(
            mine_block(&block, 65),
            None,
            "Expected None for excessive difficulty"
        );
    }
}
