use rand::rngs::OsRng;
use rand::RngCore;

fn main() {
    let mut key = [0u8; 16]; // 32-byte private key
    OsRng.fill_bytes(&mut key);
    let hex_key = hex::encode(&key);
    println!("Private key: {:?}", key);
    println!("This is the  Hex key :{} ",hex_key);
}