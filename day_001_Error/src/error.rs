use std::fmt;
use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
enum BlockChainError {
    InvalidTransaction(String),
    InsufficientFunds(u64),
    CryptoFailure(String),
    NetworkError(String),
}




#[derive(Debug)]
#[derive(Clone)]
struct Transaction{
    sender:String,
    receiver:String,
    amount :u64,
    signature:String
}



impl fmt::Display for BlockChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockChainError::InvalidTransaction(msg) => write!(f, "Invalid Transaction: {}", msg),
            BlockChainError::InsufficientFunds(deficit) => {
                write!(f, "Not enough balance. Required deficit: {}", deficit)
            }
            BlockChainError::CryptoFailure(msg) => write!(f, "Crypto Failure: {}", msg),
            BlockChainError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
        }
    }
}

impl Error for BlockChainError {}

fn validate_balance(balance: u64, required: u64) -> Result<(), BlockChainError> {
    // ()  ---> it means on success the function returns nothing
    if balance < required {
        return Err(BlockChainError::InsufficientFunds(required - balance));
    }
    Ok(())
}

fn validate_transaction(tx:&Transaction  , sender_balance:u64)->Result<Transaction , BlockChainError>{
    // check ]
    if tx.amount == 0 {
        return  Err(BlockChainError::InvalidTransaction("Amoount must be positive".to_string()));

    }

    // check the sufficient funds
    if sender_balance < tx.amount {
        return  Err(BlockChainError::InsufficientFunds(tx.amount- sender_balance));

    }
      // Check the signature
    if tx.signature !="valid_sig"{
        return Err(BlockChainError::CryptoFailure("Invalid Trasanction ".to_string()));

}

// If all  good then return ok
 Ok(tx.clone())

}


fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {


    let tx = Transaction{
        sender :"Everesty".to_string(),
        receiver:"Paudel".to_string(),
        amount :50,
        signature:"valid_sig".to_string(),


    };
    let balance = 50;

  match validate_transaction(&tx, balance){
   Ok(valid_tx) => println!("Valid transaction: {:?}", valid_tx),
    Err(e)=>println!("Expected failure :{}",e)
  }


   match validate_balance(120, 20){
    Ok(_)=> println!("Balance checked passed"),
    Err(err)=>println!("Balance error :{}",err)
   }

    match read_file("fdkfj.txt") {
        Ok(content) => println!("File contents: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}