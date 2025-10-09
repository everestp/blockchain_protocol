

use tokio::time::{sleep, Duration};

#[tokio::main]
 async  fn main() {

println!("Starting node...");
 fetch_block_data().await;
  fetch_block_data().await;
 println!("Node finidhed")

 }


 async  fn fetch_block_data(){
    println!("Fetching block data");
    sleep(Duration::from_secs(1)).await;      //simultate the network  delay
    println!("Blocked data recieed")
 }
