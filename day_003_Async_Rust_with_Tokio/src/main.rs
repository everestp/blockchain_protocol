

use reqwest::Client;
use tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};



// define the struct  for JSON-RPC request/response
#[derive(Serialize ,Debug)]
struct  RpcRequest {
    jsonrpc:String,
    id :u64,
    method:String,
    params:Vec<serde_json::Value>
}
#[derive(Deserialize, Debug)]
struct RpcResponse {
   jsonrpc:String,
   result:BlockHashResult
}



#[derive(Deserialize, Debug)]
struct  BlockHashResult {
   value:BlockHashValue
}



#[derive(Deserialize, Debug)]
struct  BlockHashValue {
   blockhash:String
}

#[tokio::main]
 async  fn main() ->Result<() , reqwest::Error> {
    let client = Client::new();
    
    let response =  client
    .get("https://jsonplaceholder.typicode.com/posts/1")
    .send()
    .await?
    .text()
    .await;
   println!("Response: {}",response.unwrap() );
println!("Starting node...");
 fetch_block_data().await;
  fetch_block_data().await;
 println!("Node finidhed");
 Ok(())

 }


 async  fn fetch_block_data(){
    println!("Fetching block data");
    sleep(Duration::from_secs(1)).await;      //simultate the network  delay
    println!("Blocked data recieed")
 }


