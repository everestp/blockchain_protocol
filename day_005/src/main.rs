use serde::{Deserialize, Serialize};



#[derive(Serialize ,Deserialize ,Debug)]
struct Transaction{
    id:u32,
    amount:u32,
    sender:String
}

#[derive(Serialize ,Deserialize,Debug)]
struct Block{
    id:u32,
    timestamp:u64,
    transaction:Vec<Transaction>
}



fn main()->Result<() ,serde_json::Error>{
    let tx= Transaction{id:1 ,amount:100 ,sender:String::from("Everest")};

    let block = Block{
        id:1 ,
        timestamp:1631234567,
        transaction:vec![
    Transaction {id:1 ,amount:100 ,sender:String::from("Everest")},
    Transaction {id:1 ,amount:1030 ,sender:String::from("{Paudel}")}
        ]
    };


let serialized1 = serde_json::to_string_pretty(&block)?;
println!("Serialized Block :{:?}",serialized1);

let deserialized:Block = serde_json::from_str(&serialized1)? ;
println!("Deserialized Block :{:?}",deserialized);





let serialized = serde_json::to_string(&block)?;
println!("Seralized Json {:?}",serialized);

let deserialize2:Transaction = serde_json::from_str(&serialized)?;
println!("Deserialzed :{:?}",deserialize2);
assert_eq!(tx.sender ,deserialize2.sender);



    Ok(())
}
