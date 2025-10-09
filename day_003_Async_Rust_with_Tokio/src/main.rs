use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};

// =========================
// JSON-RPC Request Struct
// =========================
#[derive(Serialize, Debug)]
struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Vec<serde_json::Value>,
}

// =========================
// Response for getLatestBlockhash
// =========================
#[derive(Deserialize, Debug)]
struct BlockhashResponse {
    jsonrpc: String,
    result: BlockHashResult,
}

#[derive(Deserialize, Debug)]
struct BlockHashResult {
    value: BlockHashValue,
}

#[derive(Deserialize, Debug)]
struct BlockHashValue {
    blockhash: String,
}

// =========================
// Response for getBalance
// =========================
#[derive(Deserialize, Debug)]
struct BalanceResponse {
    jsonrpc: String,
    result: BalanceResult,
}

#[derive(Deserialize, Debug)]
struct BalanceResult {
    context: Context,
    value: u64,
}

#[derive(Deserialize, Debug)]
struct Context {
    slot: u64,
}

// =========================
// Main Async Function
// =========================
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "https://api.devnet.solana.com";

    // -------------------------
    // JSON-RPC request to get latest blockhash
    // -------------------------
    let block_request = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "getLatestBlockhash".to_string(),
        params: vec![json!({"commitment": "confirmed"})],
    };

    // JSON-RPC request to get balance of a specific account
    let balance_request = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 2,
        method: "getBalance".to_string(),
        params: vec![json!("5f9xWaxoH6gNcU5uwjNeqBChMNhsbbf6Lsnh9w8ejoD9")],
    };

    // -------------------------
    // Send both requests concurrently using Tokio
    // tokio::join! allows multiple async tasks to run simultaneously
    // -------------------------
    let (block_resp, balance_resp) = tokio::join!(
        client.post(url).json(&block_request).send(),
        client.post(url).json(&balance_request).send()
    );

    // -------------------------
    // Parse JSON responses into strongly typed structs
    // -------------------------
    let block_result = block_resp?.json::<BlockhashResponse>().await?;
  let balance_result = balance_resp?.json::<BalanceResponse>().await?;
println!(
    "Balance: {} lamports (slot: {})",
    balance_result.result.value,
    balance_result.result.context.slot
);


  println!(
    "Balance: {} lamports (slot: {})",
    balance_result.result.value,
    balance_result.result.context.slot
);

    // -------------------------
    // Example of a simple GET request to a placeholder API
    // Demonstrates typical async HTTP call with Reqwest
    // -------------------------
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {}", response);

    // -------------------------
    // Simulate fetching block data asynchronously with delay
    // -------------------------
    fetch_block_data().await;
    fetch_block_data().await;

    Ok(())
}

// ==========================
// Simulated async function to fetch block data
// Shows non-blocking delay using Tokio's sleep
// ==========================
async fn fetch_block_data() {
    println!("Fetching block data...");
    sleep(Duration::from_secs(1)).await; // simulate network delay
    println!("Block data received!");
}
