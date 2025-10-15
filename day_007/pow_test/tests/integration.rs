// -----------------------------------------
// Integration Test: Full Mining Workflow
// -----------------------------------------
// This test performs a full mining cycle on a sample block:
// - Creates a block
// - Mines it with a minimal difficulty
// - Computes its hash
// - Asserts that the hash meets the expected difficulty
// -----------------------------------------

use pow_test::{Block, mine_block, compute_hash};

#[test]
fn test_full_mining() {
    // Step 1: Create a test block
    let block = Block {
        id: 1,           // Arbitrary block ID
        nonce: 0,        // Start with nonce = 0
        data: String::from("integration_test"), // Sample payload
    };

    // Step 2: Mine the block with minimal difficulty (fast for test)
    let nonce = mine_block(&block, 1).expect("Mining failed");

    // Step 3: Update the block with the mined nonce
    let mut mined_block = block;
    mined_block.nonce = nonce;

    // Step 4: Compute the final hash
    let hash = compute_hash(&mined_block);

    // Step 5: Verify that hash meets the difficulty requirement
    assert!(
        hash.starts_with("0"),
        "Hash does not meet difficulty: got {}",
        hash
    );

    // Optional: Print mined hash for debugging
    println!("Mined block nonce: {}, hash: {}", nonce, hash);
}
