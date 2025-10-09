

use reqwest::Client;
use tokio::time::{sleep, Duration};







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


