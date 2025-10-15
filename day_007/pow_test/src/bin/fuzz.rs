// -----------------------------------------
// Fuzz Testing for Proof-of-Work Mining
// -----------------------------------------
// Uses AFL (American Fuzzy Lop) to feed arbitrary inputs
// to the mining function and test its robustness.
// -----------------------------------------

#[macro_use]
extern crate afl;

use pow_test::{Block, mine_block};

fn main() {
    // Start AFL fuzzing loop
    fuzz!(|data: &[u8]| {
        // Attempt to interpret input bytes as UTF-8 string
        if let Ok(data_str) = std::str::from_utf8(data) {
            // Construct a block using fuzz input
            let block = Block {
                // Use the first byte of input as block ID, default to 1 if empty
                id: data.first().map(|&b| b as u32).unwrap_or(1),
                
                // Start nonce at 0
                nonce: 0,
                
                // Use the fuzz input string as block data
                data: data_str.to_string(),
            };

            // Mine the block with minimal difficulty (1) to avoid long fuzz runs
            let _ = mine_block(&block, 1);

            // We ignore the result, as the goal is just to detect crashes or panics
        }
    });
}
