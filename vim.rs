use blockchain_traits::{Block, PoWValidator, PoSValidator, Validator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let block = Block {
        id: 1,
        nonce: 10,
        data: String::from("1000"),
    };

    let pow_validator = PoWValidator { difficulty: 1 };
    let pos_validator = PoSValidator { min_stake: 500 };

    println!("PoW valid: {}", pow_validator.validate(&block));
    println!("PoS valid: {}", pos_validator.validate(&block));

    Ok(())
}



fd 
