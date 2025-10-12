use serde::{Deserialize, Serialize};



#[derive(Serialize ,Deserialize ,Debug)]
struct Transaction{
    id:u32,
    amount:u64,
    sender:String
}



fn main()->Result<() ,serde_json::Error>{
    let tx= Transaction{id:1 ,amount:100 ,sender:String::from("Everest")};
let serialized = serde_json::to_string(&tx)?;
println!("Seralized Json {:?}",serialized);

let deserialize:Transaction = serde_json::from_str(&serialized)?;
println!("Deserialzed :{:?}",deserialize);
assert_eq!(tx.sender ,deserialize.sender);



    Ok(())
}
