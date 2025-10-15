// Import the mining module (pow_test) that contains Block, mining logic, and hash computation
use pow_test::{Block, mine_block, compute_hash};

fn main() {
    // -----------------------------
    // Step 1: Create a new block
    // -----------------------------
    let block = Block {
        id: 1, // Block ID
        nonce: 0, // Initial nonce before mining
        data: String::from("Hello, blockchain!"), // Block payload
    };

    // -----------------------------
    // Step 2: Set the mining difficulty
    // -----------------------------
    // Difficulty = number of leading zeros required in hash
    let difficulty = 2;

    // -----------------------------
    // Step 3: Attempt to mine the block
    // -----------------------------
    match mine_block(&block, difficulty) {
        Some(nonce) => {
            // If mining succeeds, update the block's nonce
            let mut mined_block = block.clone();
            mined_block.nonce = nonce;

            // Compute the final hash of the mined block
            let hash = compute_hash(&mined_block);

            // Display the mined block details
            println!("Mined block with nonce: {}, hash: {}", nonce, hash);
        }
        None => {
            // Mining failed due to invalid difficulty or oversized data
            println!("Mining failed: invalid difficulty or data");
        }
    }
}
