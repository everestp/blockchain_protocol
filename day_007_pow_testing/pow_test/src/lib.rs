// -----------------------------------------
// Blockchain Mining Module
// -----------------------------------------
// Author: Everest Paudel
// Description: A simple and educational example of a proof-of-work
// mining system in Rust using SHA-256 hashing. This module includes
// the `Block` structure, hash computation, and mining logic with safety
// limits for difficulty and data size.
// -----------------------------------------

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

// -----------------------------------------
// Block Structure
// -----------------------------------------
// Represents a basic blockchain block containing an ID,
// nonce (used for proof-of-work), and arbitrary data payload.
// The block can be serialized and hashed.
// -----------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    /// Unique identifier for the block (e.g., its position in the chain)
    pub id: u32,
    
    /// Nonce value used for proof-of-work mining
    pub nonce: u64,
    
    /// Data or payload stored in the block
    pub data: String,
}

// -----------------------------------------
// Mining Function
// -----------------------------------------
// Attempts to find a valid nonce that produces a hash
// starting with `difficulty` number of leading zeros.
// Returns `Some(nonce)` if successful, or `None` if mining fails
// (due to excessive difficulty or invalid block data).
// -----------------------------------------
pub fn mine_block(block: &Block, difficulty: usize) -> Option<u64> {
    // Safety limits:
    // - Prevent extreme difficulty that could hang the miner
    // - Limit data size to prevent unnecessary computation or abuse
    if difficulty > 64 || block.data.len() > 1000 {
        return None;
    }

    // Target hash prefix (e.g., "0000" for difficulty 4)
    let target = "0".repeat(difficulty);

    // Brute-force search for a valid nonce
    for nonce in 0..=u64::MAX {
        let mut test_block = block.clone();
        test_block.nonce = nonce;

        // Compute the hash for this candidate nonce
        let hash = compute_hash(&test_block);

        // Check if hash meets the difficulty requirement
        if hash.starts_with(&target) {
            return Some(nonce);
        }
    }

    // If no valid nonce is found (theoretically unreachable)
    None
}

// -----------------------------------------
// Hash Computation Function
// -----------------------------------------
// Serializes the block and computes its SHA-256 hash.
// The resulting hash is returned as a lowercase hexadecimal string.
// -----------------------------------------
pub fn compute_hash(block: &Block) -> String {
    // Serialize the block into JSON format
    let serialized = serde_json::to_string(block)
        .expect("Serialization failed");

    // Initialize SHA-256 hasher and process the block data
    let mut hasher = Sha256::new();
    hasher.update(serialized);

    // Return the final hash as a hex string
    format!("{:x}", hasher.finalize())
}

// -----------------------------------------
// Unit Tests
// -----------------------------------------
// These tests verify the correctness and safety of mining logic.
// Run using: `cargo test`
// -----------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine_block_valid() {
        // Create a simple test block
        let block = Block {
            id: 1,
            nonce: 0,
            data: String::from("test"),
        };

        // Attempt to mine with low difficulty (for quick test)
        let nonce = mine_block(&block, 2).expect("Mining failed");

        // Verify that mined block's hash meets difficulty
        let mut mined_block = block.clone();
        mined_block.nonce = nonce;
        let hash = compute_hash(&mined_block);
        assert!(
            hash.starts_with("00"),
            "Hash does not meet difficulty: got {}",
            hash
        );
    }

    #[test]
    fn test_mine_block_too_difficult() {
        // Difficulty too high should immediately fail
        let block = Block {
            id: 1,
            nonce: 0,
            data: String::from("test"),
        };
        assert_eq!(
            mine_block(&block, 65),
            None,
            "Expected None for excessive difficulty"
        );
    }

    #[test]
    fn test_mine_block_oversized_data() {
        // Block with oversized data (>1000 chars) should be rejected
        let block = Block {
            id: 1,
            nonce: 0,
            data: String::from_utf8(vec![b'a'; 1001]).unwrap(),
        };
        assert_eq!(
            mine_block(&block, 1),
            None,
            "Expected None for oversized data"
        );
    }
}
