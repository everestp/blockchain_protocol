//! Simple KeyPair example (educational).
//!
//! - Generates a 32-byte random "private key" using the OS CSPRNG.
//! - Derives a "public key" by hashing the private key with SHA-256.
//! - Serializes / deserializes the KeyPair with serde_json.
//! - Provides `verify()` to re-compute hash(private_key) and compare to stored public key.
//!
//! NOTE: Hashing the private key is **not** how real asymmetric public keys are generated.
//! For real keypairs (able to sign & verify), use an asymmetric scheme like Ed25519 (ed25519-dalek).

use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use hex;

/// A simple KeyPair struct holding hex-encoded keys.
///
/// - `private_key` is 32 random bytes encoded as hex (64 hex chars).
/// - `public_key` is SHA-256(private_key) encoded as hex (64 hex chars).
#[derive(Serialize, Deserialize, Debug, Clone)]
struct KeyPair {
    /// Hex-encoded 32-byte private key (must be kept secret in real use).
    private_key: String,

    /// Hex-encoded SHA-256(private_key). Educational only — not a real public key.
    public_key: String,
}

impl KeyPair {
    /// Generate a new KeyPair.
    ///
    /// - Uses OS RNG for cryptographically-secure randomness.
    /// - Encodes private key and public key as hex strings for easy storage/display.
    fn new() -> Self {
        // Fill a 32-byte buffer with secure random bytes.
        let mut private_key = [0u8; 32];
        let mut rng = OsRng; // create instance of OS RNG
        rng.fill_bytes(&mut private_key);

        // Convert private key bytes to hex string for storage/display.
        let hex_private = hex::encode(&private_key);

        // Derive a "public key" by hashing the private key with SHA-256.
        // IMPORTANT: This is NOT a proper asymmetric public key derivation.
        let mut hasher = Sha256::new();
        hasher.update(&private_key);
        let public_key_bytes = hasher.finalize();
        let hex_public = hex::encode(&public_key_bytes);

        KeyPair {
            private_key: hex_private,
            public_key: hex_public,
        }
    }

    /// Verify that `public_key == SHA256(private_key)`.
    ///
    /// Returns `true` when the stored public key matches the recomputed hash.
    fn verify(&self) -> bool {
        // Decode hex private key back to bytes
        let private_bytes = match hex::decode(&self.private_key) {
            Ok(b) => b,
            Err(_) => return false, // invalid hex stored in private_key
        };

        // Compute SHA-256(private_bytes)
        let mut hasher = Sha256::new();
        hasher.update(&private_bytes);
        let computed = hasher.finalize();

        // Compare hex-encoded computed hash with stored public_key
        hex::encode(computed) == self.public_key
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new KeyPair
    let key_pair = KeyPair::new();

    // Pretty-print the KeyPair (Debug)
    println!("Generated KeyPair: {:#?}", key_pair);

    // Verify the pair locally
    if key_pair.verify() {
        println!("✔ KeyPair verification succeeded (public = SHA256(private)).");
    } else {
        println!("✖ KeyPair verification failed!");
    }

    // Serialize to pretty JSON
    let serialized = serde_json::to_string_pretty(&key_pair)?;
    println!("\nSerialized KeyPair (JSON):\n{}", serialized);

    // Deserialize back from JSON
    let deserialized: KeyPair = serde_json::from_str(&serialized)?;
    println!("\nDeserialized KeyPair: {:#?}", deserialized);

    // Basic sanity check
    assert_eq!(key_pair.private_key, deserialized.private_key);
    assert_eq!(key_pair.public_key, deserialized.public_key);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_pair_generation() {
        let kp = KeyPair::new();
        // Each hex string for 32 bytes -> 64 hex characters
        assert_eq!(kp.private_key.len(), 64);
        // SHA-256 output is 32 bytes -> 64 hex characters
        assert_eq!(kp.public_key.len(), 64);
    }

    #[test]
    fn test_key_verification() {
        let kp = KeyPair::new();
        assert!(kp.verify());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let kp = KeyPair::new();
        let serialized = serde_json::to_string(&kp).expect("Serialization failed");
        let deserialized: KeyPair =
            serde_json::from_str(&serialized).expect("Deserialization failed");
        assert_eq!(kp.private_key, deserialized.private_key);
        assert_eq!(kp.public_key, deserialized.public_key);
    }
}
