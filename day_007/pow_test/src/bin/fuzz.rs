#[macro_use]
extern crate afl;
use pow_test::{Block, mine_block};

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(data_str) = std::str::from_utf8(data) {
            let block = Block {
                id: 1,
                nonce: 0,
                data: data_str.to_string(),
            };
            let _ = mine_block(&block, 1); // Test robustness
        }
    });
}