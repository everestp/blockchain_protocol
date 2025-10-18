# Day 11: Enums & Pattern Matching for Blockchain States

Welcome to **Day 11** of your **Rust-for-Blockchain Journey**! This comprehensive README guides you through mastering **enums and pattern matching** to model blockchain states, such as transaction statuses (`Pending`, `Confirmed`, `Failed`) and block types (`Genesis`, `Regular`, `PoSBlock`). Building on all previous days—concurrency (Day 2), async programming (Day 3), procedural macros (Day 4), serialization (Day 5), unsafe Rust (Day 6), testing (Day 7), key generation (Day 8), traits (Day 9), and lifetimes (Day 10)—this module integrates every key blockchain aspect into a cohesive system. You’ll design a blockchain node that uses enums to manage transaction and block states, pattern matching to handle state transitions, and integrates consensus, cryptography, networking, and state management for a robust blockchain simulation.

The practice exercise involves building a **mini-blockchain node** with a lifetime-annotated mempool (Day 10), pluggable validators (Day 9), async block fetching (Day 3), and comprehensive tests (Day 7). You’ll model transaction states and block variants, ensuring type-safe, memory-safe, and performant blockchain logic. Create a new Cargo project with `cargo new blockchain_enums` to get started.

---

## 🎯 Objectives
- Use **enums** to model blockchain transaction states (`Pending`, `Confirmed`, `Failed`) and block types (`Genesis`, `Regular`, `PoSBlock`).
- Apply **pattern matching** to process states exhaustively, ensuring robust state transitions.
- Integrate **consensus mechanisms** (Day 9) like Proof-of-Work (PoW) and Proof-of-Stake (PoS) with block validation.
- Incorporate **cryptographic signatures** (Day 8) for secure block validation.
- Manage **long-lived chain states** with lifetimes (Day 10) in a mempool and blockchain ledger.
- Use **async programming** (Day 3) for block fetching and **concurrency** (Day 2) for transaction processing.
- Apply **serialization** (Day 5) for block and transaction persistence, and **hashing** (Day 4) for integrity.
- Ensure **memory safety** with safe Rust, replacing unsafe code (Day 6) where possible.
- Write **comprehensive tests** (Day 7) to verify state transitions and validation logic.

## 📋 Prerequisites
- **Rust Basics**: Ownership, borrowing, references, modules, and error handling.
- **Previous Days**:
  - **Day 2**: Concurrency with threads and channels for parallel transaction processing.
  - **Day 3**: Async programming with Tokio for network operations.
  - **Day 4**: Procedural macros for generating hashing code.
  - **Day 5**: Serialization with `serde` for data persistence and transmission.
  - **Day 6**: Unsafe Rust for low-level mempool operations (used sparingly).
  - **Day 7**: Testing with `#[test]` for robust verification.
  - **Day 8**: Key generation with `ed25519-dalek` for cryptographic signatures.
  - **Day 9**: Traits for modular validator design (PoW, PoS).
  - **Day 10**: Lifetimes for managing long-lived chain states and mempools.
- **Dependencies**: `serde`, `serde_json`, `sha2`, `tokio`, `ed25519-dalek`, `rand`, `hex`.

## 🛠 Setup
1. Create a new Cargo project:
   ```bash
   cargo new blockchain_enums
   cd blockchain_enums
   ```
2. Update `Cargo.toml` to include all necessary dependencies:
<xaiArtifact artifact_id="268b9123-016a-4e13-b820-7a4c6e259ad8" artifact_version_id="c36238b7-753e-4e98-a3c2-4f9fca260a00" title="Cargo.toml" contentType="text/toml">
[package]
name = "blockchain_enums"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10"
tokio = { version = "1.0", features = ["full"] }
ed25519-dalek = "1.0"
rand = "0.8"
hex = "0.4"
</xaiArtifact>

3. Implement the code in `src/lib.rs` and `src/main.rs` as provided below.

---

## 📚 Comprehensive Blockchain Learning Plan

This plan covers all blockchain aspects—**data structures**, **consensus**, **cryptography**, **networking**, **state management**, **persistence**, and **testing**—using enums and pattern matching as the core mechanism for state handling.

### Step 1: Modeling Blockchain States with Enums
Enums are ideal for representing distinct blockchain states, such as transaction statuses or block types. Pattern matching ensures these states are processed exhaustively, preventing invalid transitions.

- **Why Enums?**
  - Enums define a fixed set of variants, perfect for blockchain states like `Pending`, `Confirmed`, or `Failed` transactions, and `Genesis`, `Regular`, or `PoSBlock` blocks.
  - **Blockchain Relevance**: Enums model transaction lifecycles (mempool to block inclusion) and block types (genesis vs. regular), critical for consensus and ledger updates.
  - **Analogy**: Transactions are like orders in a restaurant (`Ordered`, `Served`, `Cancelled`). Enums define these states, and pattern matching handles their processing (e.g., notify customer on cancellation).
  - **Memory Safety**: Enums leverage Rust’s type system to ensure valid states, preventing bugs like processing a failed transaction as confirmed.

- **Simple Example**: Transaction state enum.
<xaiArtifact artifact_id="b6a9d614-3373-40ad-b8d0-dfa5ec0a01e5" artifact_version_id="f27230c9-fc52-4b4e-99af-04c7d5676819" title="simple_transaction_state.rs" contentType="text/rust">
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionState {
    Pending { id: u32, amount: u64 },
    Confirmed { id: u32, amount: u64, block_id: u32 },
    Failed { id: u32, reason: String },
    Rejected { id: u32, error_code: u32 },
}

pub fn process_transaction(state: &TransactionState) -> String {
    match state {
        TransactionState::Pending { id, amount } => format!("Transaction {} (amount: {}) is pending", id, amount),
        TransactionState::Confirmed { id, amount, block_id } => format!("Transaction {} (amount: {}) confirmed in block {}", id, amount, block_id),
        TransactionState::Failed { id, reason } => format!("Transaction {} failed: {}", id, reason),
        TransactionState::Rejected { id, error_code } => format!("Transaction {} rejected with error code {}", id, error_code),
    }
}
</xaiArtifact>
  - **Run**: Add to `src/lib.rs` and test in `main.rs`:
    ```rust
    use blockchain_enums::TransactionState;

    fn main() {
        let states = vec![
            TransactionState::Pending { id: 1, amount: 500 },
            TransactionState::Confirmed { id: 2, amount: 1000, block_id: 10 },
            TransactionState::Failed { id: 3, reason: String::from("Insufficient funds") },
            TransactionState::Rejected { id: 4, error_code: 404 },
        ];
        for state in states {
            println!("{}", process_transaction(&state));
        }
    }
    ```
  - **Output**:
    ```
    Transaction 1 (amount: 500) is pending
    Transaction 2 (amount: 1000) confirmed in block 10
    Transaction 3 failed: Insufficient funds
    Transaction 4 rejected with error code 404
    ```
  - **Blockchain Tie-In**: Models a transaction’s lifecycle in a mempool (Day 6, Day 10) or ledger, integrating serialization (Day 5) for persistence.

- **Mini-Exercise**:
  - Add a test for the `Rejected` variant in `src/lib.rs`:
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_rejected_transaction() {
            let rejected = TransactionState::Rejected { id: 4, error_code: 404 };
            assert_eq!(process_transaction(&rejected), "Transaction 4 rejected with error code 404");
        }
    }
    ```

### Step 2: Modeling Blocks with Enums
Blockchain nodes handle different block types (e.g., `Genesis` for chain initialization, `Regular` for PoW, `PoSBlock` for PoS). Enums model these types, and pattern matching processes them with validators (Day 9).

- **Why Block Enums?**
  - Enums distinguish block types, ensuring type-safe validation and processing.
  - **Blockchain Relevance**: Genesis blocks initialize the chain (e.g., Bitcoin’s genesis block), while regular and PoS blocks extend it. Pattern matching handles their unique validation rules.
  - **Ties to Consensus**: Integrates PoW and PoS validators (Day 9) for modular consensus.

- **Example**: Block enum with PoW validator.
<xaiArtifact artifact_id="a37a2f8c-03d8-41d9-a0fe-b9cb6a256675" artifact_version_id="86506110-082d-42c7-af09-025b96a43908" title="block_enum.rs" contentType="text/rust">
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Block {
    Genesis { id: u32, data: String },
    Regular { id: u32, nonce: u64, data: String, prev_hash: String },
    PoSBlock { id: u32, stake: u64, data: String },
}

pub trait Validator {
    fn validate(&self, block: &Block) -> bool;
}

pub struct PoWValidator {
    pub difficulty: usize,
}

impl Validator for PoWValidator {
    fn validate(&self, block: &Block) -> bool {
        match block {
            Block::Genesis { .. } => true,
            Block::Regular { id, nonce, data, prev_hash } => {
                let hash = compute_hash(&format!("{}:{}:{}", id, nonce, data));
                hash.starts_with(&"0".repeat(self.difficulty)) && prev_hash.len() == 64
            }
            Block::PoSBlock { .. } => false,
        }
    }
}

pub fn compute_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
</xaiArtifact>
  - **Run**: Add to `src/lib.rs` and test in `main.rs`:
    ```rust
    use blockchain_enums::{Block, PoWValidator, Validator};

    fn main() {
        let genesis = Block::Genesis { id: 0, data: String::from("genesis") };
        let regular = Block::Regular { id: 1, nonce: 10, data: String::from("data"), prev_hash: "0".repeat(64) };
        let validator = PoWValidator { difficulty: 1 };
        println!("Genesis valid: {}", validator.validate(&genesis));
        println!("Regular valid: {}", validator.validate(&regular));
    }
    ```

- **Mini-Exercise**:
  - Add a `PoSValidator` for `PoSBlock`:
    ```rust
    pub struct PoSValidator { pub min_stake: u64 }
    impl Validator for PoSValidator {
        fn validate(&self, block: &Block) -> bool {
            match block {
                Block::Genesis { .. } => true,
                Block::Regular { .. } => false,
                Block::PoSBlock { stake, .. } => stake >= &self.min_stake,
            }
        }
    }
    ```

### Step 3: Integrating Lifetimes and State Management
Blockchain nodes manage long-lived data (e.g., mempools, chain states) with references, requiring lifetimes (Day 10). We’ll combine enums with lifetimes to track transaction states and integrate with a chain state.

- **Example**: Lifetime-annotated transaction states.
<xaiArtifact artifact_id="8fb3b6f9-3977-4310-a863-3b474f8c7009" artifact_version_id="2c26bcd1-b546-4565-98ed-6d7c90bd8742" title="lifetime_transaction_state.rs" contentType="text/rust">
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: u32,
    pub amount: u64,
    pub data: String,
}

#[derive(Debug)]
pub enum TransactionState<'a> {
    Pending { tx: &'a Transaction },
    Confirmed { tx: &'a Transaction, block_id: u32 },
    Failed { tx: &'a Transaction, reason: String },
}

pub struct ChainState {
    pub balances: std::collections::HashMap<u32, u64>,
    pub last_block_id: u32,
}

pub struct Mempool<'a> {
    states: Vec<TransactionState<'a>>,
}
</xaiArtifact>

### Step 4: Cryptographic Signatures
Add a `SignatureValidator` (Day 8) to validate blocks with cryptographic signatures, ensuring authenticity.

- **Example**:
<xaiArtifact artifact_id="b60b6e2f-3476-4a94-bb9a-8575261f621b" artifact_version_id="a8b98ef3-0989-42a6-86e2-09aa93da7b6c" title="signature_validator.rs" contentType="text/rust">
use ed25519_dalek::{Keypair, Signer, Verifier, PublicKey};
use rand::rngs::OsRng;

pub struct SignatureValidator {
    pub public_key: PublicKey,
}

impl Validator for SignatureValidator {
    fn validate(&self, block: &Block) -> bool {
        match block {
            Block::Genesis { .. } => true,
            Block::Regular { id, nonce, data, .. } => {
                let message = format!("{}:{}:{}", id, nonce, data);
                let signature = hex::decode(data).ok()?;
                self.public_key.verify(message.as_bytes(), &signature.into()).is_ok()
            }
            Block::PoSBlock { .. } => false,
        }
    }
}
</xaiArtifact>

### Step 5: Async and Concurrent Processing
Use async (Day 3) for block fetching and concurrency (Day 2) for transaction processing.

- **Example**: Async block fetching.
<xaiArtifact artifact_id="c8dee1e2-4e64-4737-b58c-bb671e12c6d7" artifact_version_id="c17bcd25-5d96-4fa0-a657-652b999fa502" title="async_fetch.rs" contentType="text/rust">
async fn fetch_block() -> Block {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Block::PoSBlock { id: 2, stake: 1000, data: String::from("fetched") }
}
</xaiArtifact>

### Step 6: Practice Exercise - Mini-Blockchain Node
**Goal**: Build a blockchain node with enum-based transaction and block states, lifetime-annotated mempools, pluggable validators, async block fetching, and cryptographic signatures.

- **Full Code** (in `src/lib.rs`):
<xaiArtifact artifact_id="23b5e164-bd94-403e-82e6-ec6574170a14" artifact_version_id="e8a94c0d-c3f2-4764-a688-005bb03048da" title="lib.rs" contentType="text/rust">
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use ed25519_dalek::{Keypair, Signer, Verifier, PublicKey};
use rand::rngs::OsRng;
use hex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: u32,
    pub amount: u64,
    pub data: String,
}

#[derive(Debug)]
pub enum TransactionState<'a> {
    Pending { tx: &'a Transaction },
    Confirmed { tx: &'a Transaction, block_id: u32 },
    Failed { tx: &'a Transaction, reason: String },
    Rejected { tx: &'a Transaction, error_code: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Block {
    Genesis { id: u32, data: String },
    Regular { id: u32, nonce: u64, data: String, prev_hash: String },
    PoSBlock { id: u32, stake: u64, data: String },
}

pub trait Validator {
    fn validate(&self, block: &Block) -> bool;
}

pub struct PoWValidator {
    pub difficulty: usize,
}

impl Validator for PoWValidator {
    fn validate(&self, block: &Block) -> bool {
        match block {
            Block::Genesis { .. } => true,
            Block::Regular { id, nonce, data, prev_hash } => {
                let hash = compute_hash(&format!("{}:{}:{}", id, nonce, data));
                hash.starts_with(&"0".repeat(self.difficulty)) && prev_hash.len() == 64
            }
            Block::PoSBlock { .. } => false,
        }
    }
}

pub struct PoSValidator {
    pub min_stake: u64,
}

impl Validator for PoSValidator {
    fn validate(&self, block: &Block) -> bool {
        match block {
            Block::Genesis { .. } => true,
            Block::Regular { .. } => false,
            Block::PoSBlock { stake, .. } => stake >= &self.min_stake,
        }
    }
}

pub struct SignatureValidator {
    pub public_key: PublicKey,
}

impl Validator for SignatureValidator {
    fn validate(&self, block: &Block) -> bool {
        match block {
            Block::Genesis { .. } => true,
            Block::Regular { id, nonce, data, .. } => {
                let message = format!("{}:{}:{}", id, nonce, data);
                let signature = hex::decode(data).ok()?;
                self.public_key.verify(message.as_bytes(), &signature.into()).is_ok()
            }
            Block::PoSBlock { .. } => false,
        }
    }
}

pub struct ChainState {
    pub balances: HashMap<u32, u64>,
    pub last_block_id: u32,
}

pub struct Mempool<'a> {
    states: Vec<TransactionState<'a>>,
    validator: Box<dyn Validator>,
}

impl<'a> Mempool<'a> {
    pub fn new(validator: Box<dyn Validator>) -> Self {
        Mempool { states: Vec::new(), validator }
    }

    pub fn add_transaction(&mut self, tx: &'a Transaction, block: &Block, state: &ChainState) -> bool {
        if self.validator.validate(block) && block_id_valid(block, state) {
            self.states.push(TransactionState::Confirmed { tx, block_id: match block {
                Block::Genesis { id, .. } => *id,
                Block::Regular { id, .. } => *id,
                Block::PoSBlock { id, .. } => *id,
            }});
            true
        } else {
            self.states.push(TransactionState::Failed { tx, reason: String::from("Invalid block or ID") });
            false
        }
    }

    pub fn count_confirmed(&self) -> usize {
        self.states.iter().filter(|state| matches!(state, TransactionState::Confirmed { .. })).count()
    }
}

pub fn compute_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn block_id_valid(block: &Block, state: &ChainState) -> bool {
    match block {
        Block::Genesis { id, .. } => *id == 0,
        Block::Regular { id, .. } | Block::PoSBlock { id, .. } => *id == state.last_block_id + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_states() {
        let tx = Transaction { id: 1, amount: 500, data: String::from("tx") };
        let states = vec![
            TransactionState::Pending { tx: &tx },
            TransactionState::Confirmed { tx: &tx, block_id: 1 },
            TransactionState::Failed { tx: &tx, reason: String::from("Invalid") },
            TransactionState::Rejected { tx: &tx, error_code: 404 },
        ];
        assert_eq!(process_transaction(&states[0]), "Transaction 1 (amount: 500) is pending");
        assert_eq!(process_transaction(&states[1]), "Transaction 1 (amount: 500) confirmed in block 1");
        assert_eq!(process_transaction(&states[2]), "Transaction 1 failed: Invalid");
        assert_eq!(process_transaction(&states[3]), "Transaction 1 rejected with error code 404");
    }

    #[test]
    fn test_pow_validator() {
        let genesis = Block::Genesis { id: 0, data: String::from("genesis") };
        let regular = Block::Regular { id: 1, nonce: 10, data: String::from("data"), prev_hash: "0".repeat(64) };
        let validator = PoWValidator { difficulty: 1 };
        assert!(validator.validate(&genesis));
        let hash = compute_hash(&format!("{}:{}:{}", regular.id, regular.nonce, regular.data));
        assert_eq!(validator.validate(&regular), hash.starts_with("0"));
    }

    #[test]
    fn test_pos_validator() {
        let pos_block = Block::PoSBlock { id: 1, stake: 1000, data: String::from("data") };
        let validator = PoSValidator { min_stake: 500 };
        assert!(validator.validate(&pos_block));
    }

    #[test]
    fn test_signature_validator() {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        let message = "1:10:data";
        let signature = keypair.sign(message.as_bytes());
        let block = Block::Regular {
            id: 1,
            nonce: 10,
            data: hex::encode(signature.to_bytes()),
            prev_hash: "0".repeat(64),
        };
        let validator = SignatureValidator { public_key: keypair.public };
        assert!(validator.validate(&block));
    }

    #[test]
    fn test_mempool() {
        let validator = Box::new(PoSValidator { min_stake: 500 });
        let mut mempool = Mempool::new(validator);
        let tx = Transaction { id: 1, amount: 500, data: String::from("tx") };
        let block = Block::PoSBlock { id: 1, stake: 1000, data: String::from("data") };
        let state = ChainState { balances: HashMap::new(), last_block_id: 0 };
        assert!(mempool.add_transaction(&tx, &block, &state));
        assert_eq!(mempool.count_confirmed(), 1);
    }
}
</xaiArtifact>

- **Main Program** (in `src/main.rs`):
<xaiArtifact artifact_id="fb26641c-5ff6-4adf-a9dc-844ab492d7fb" artifact_version_id="100308ac-1936-4980-a500-a1029d8eb926" title="main.rs" contentType="text/rust">
use blockchain_enums::{Block, Mempool, PoSValidator, Transaction, ChainState};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut balances = HashMap::new();
    balances.insert(1, 1000);
    let state = ChainState { balances, last_block_id: 0 };
    let validator = Box::new(PoSValidator { min_stake: 500 });
    let mut mempool = Mempool::new(validator);
    let tx = Transaction { id: 1, amount: 500, data: String::from("tx") };
    let block = Block::PoSBlock { id: 1, stake: 1000, data: String::from("data") };

    mempool.add_transaction(&tx, &block, &state);
    println!("Confirmed transactions: {}", mempool.count_confirmed());

    let fetched_block = fetch_block().await;
    mempool.add_transaction(&tx, &fetched_block, &state);
    println!("Confirmed transactions after async fetch: {}", mempool.count_confirmed());

    Ok(())
}

async fn fetch_block() -> Block {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Block::PoSBlock { id: 2, stake: 1000, data: String::from("fetched") }
}
</xaiArtifact>

- **Run**:
  - `cargo test` to verify all tests pass.
  - `cargo run` for output:
    ```
    Confirmed transactions: 1
    Confirmed transactions after async fetch: 2
    ```

- **Blockchain Aspects Covered**:
  - **Data Structures**: Enums for transactions and blocks, HashMap for chain state.
  - **Consensus**: PoW, PoS, and signature validators (Day 9).
  - **Cryptography**: Ed25519 signatures (Day 8) for block authenticity.
  - **Networking**: Async block fetching (Day 3).
  - **State Management**: Lifetime-annotated mempool and chain state (Day 10).
  - **Persistence**: Serialization with `serde` (Day 5).
  - **Integrity**: SHA-256 hashing (Day 4).
  - **Testing**: Comprehensive unit tests (Day 7).
  - **Safety**: Safe Rust with lifetimes, avoiding unsafe code (Day 6).
  - **Modularity**: Traits for pluggable validators (Day 9).

- **Advanced Extensions**:
  - **Concurrent Processing**: Use threads (Day 2) to process transactions:
    ```rust
    use std::thread;

    fn process_concurrently(txs: Vec<&Transaction>, block: &Block, mempool: &mut Mempool, state: &ChainState) {
        let handles: Vec<_> = txs.into_iter().map(|tx| {
            let block = block.clone();
            let state = state.clone();
            thread::spawn(move || mempool.add_transaction(tx, &block, &state))
        }).collect();
        for handle in handles {
            handle.join().unwrap();
        }
    }
    ```
  - **Byzantine Fault Tolerance (BFT)**: Add a `ByzantineBlock` variant with a quorum of signatures.

---

## 📖 Resources
- **YouTube**: Watch “Enums & Matching” by freeCodeCamp (0–15 mins) for enum and pattern matching basics.
- **GitHub**: Explore [github.com/rust-lang/book](https://github.com/rust-lang/book) chapter 6 examples (`ch06-00-enums`).
- **Docs**: Read [doc.rust-lang.org/book/ch06-00-enums.html](https://doc.rust-lang.org/book/ch06-00-enums.html) and [doc.rust-lang.org/book/ch18-00-patterns.html](https://doc.rust-lang.org/book/ch18-00-patterns.html).
- **Rust by Example**: Check [rust-by-example.github.io/enums.html](https://rust-by-example.github.io/enums.html).

## ✅ Review & Notes
- **Key Concepts**:
  - Enums model blockchain states with type safety.
  - Pattern matching ensures robust state transitions.
  - Integrates all blockchain aspects: consensus, cryptography, networking, state management, persistence, and testing.
- **Reflect**:
  - Did the node handle states correctly? Note any issues (e.g., lifetime errors, match exhaustiveness).
  - How do enums enhance blockchain design? (Answer: Type-safe state modeling and robust transitions.)
- **Journal**:
  - Summarize enums and their blockchain applications in 3–5 sentences.
  - Document a challenge (e.g., integrating signatures) and your solution.
  - Propose a project, like a full blockchain with BFT consensus.
- **GitHub**:
  - Commit: `git add . && git commit -m "Day 11: Comprehensive blockchain node with enums"`.
  - Push: `git push origin main`.

## 🚀 Next Steps
- **Extend**: Add a `ByzantineBlock` variant for BFT consensus.
- **Integrate**: Build a mock P2P network (Day 3) for block propagation.
- **Experiment**: Implement a state trie with enums for Ethereum-like state management.
- **Challenge Project**: Create a full blockchain with enum-based states, pluggable consensus, lifetime-annotated mempools, and persistent storage.

Questions? Need tailored exercises or clarifications? Let me know, and I’ll customize challenges to deepen your blockchain expertise! 🚀