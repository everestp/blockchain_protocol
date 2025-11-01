use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub nonce: u64,
    pub data: String,
}

pub trait Validator {
    fn validate(&self, block: &Block) -> bool;
}

pub struct PoWValidator {
    pub difficulty: usize,
}

impl Validator for PoWValidator {
    fn validate(&self, block: &Block) -> bool {
        let hash = compute_hash(block);
        hash.starts_with(&"0".repeat(self.difficulty))
    }
}
// hey this is the haf 
pub struct PoSValidator {
    pub min_stake: u64,
}

impl Validator for PoSValidator {
    fn validate(&self, block: &Block) -> bool {
        block.data.parse::<u64>().map_or(false, |stake| stake >= self.min_stake)
    }
}

pub fn compute_hash(block: &Block) -> String {
    let serialized = serde_json::to_string(block).expect("Serialization failed");
    let mut hasher = Sha256::new();
    hasher.update(serialized);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_validator() {
        let block = Block {
            id: 1,
            nonce: 10,
            data: String::from("test"),
        };
        let validator = PoWValidator { difficulty: 1 };
        let hash = compute_hash(&block);
        assert_eq!(validator.validate(&block), hash.starts_with("0"));
    }
dfh 
    #[test]
    fn test_pos_validator() {
        let block = Block {
            id: 1,
            nonce: 0,
            data: String::from("1000"),
        };
        let validator = PoSValidator { min_stake: 500 };
        assert!(validator.validate(&block));

        let invalid_block = Block {
            id: 2,
            nonce: 0,
            data: String::from("100"),
        };
        assert!(!validator.validate(&invalid_block));
    }
}
