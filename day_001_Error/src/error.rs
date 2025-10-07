use core::fmt;
use std::{fs::File, io::{self, Read}, sync::mpsc::RecvTimeoutError, task::Context};



#[derive(Debug)]
enum BlockChainError{
    InvalidTransaction(String),
    InsufficientFunds(u64),
    CryptoFailure(String),
    NetworkError(String)
}


impl fmt::Display for BlockChainError {
    fn fmt(&self , f:&mut fmt::Formatter)->fmt::Result {
        match self {
            BlockChainError::InvalidTransaction(msg)=>write!(f ,"Invalid Trasaction :{}",msg),
             BlockChainError::InsufficientFunds(msg)=>write!(f ,"Not enough balance:{}",msg),
              BlockChainError::CryptoFailure(msg)=>write!(f ,"Crypto Failure :{}",msg),
               BlockChainError::NetworkError(msg)=>write!(f ,"Network Error:{}",msg),

        }
    }
}

impl Error for BlockchainError {}


fn validate_balance(balance:u64 ,required:u64)->Result<(),BlockChainError>{
    //()  ---> it means om sucerss the function return nothing
        if   balance < required{
        return  Err(BlockChainError::InsufficientFunds(required - balance)));
    }
Ok(())
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents); 
    Ok(contents)

}

fn main(){
    match  read_file("fdkfj.txt") {
       Ok(content) => println!("File contents: {}", content),
        Err(e) => println!("Error: {}", e),
        
    }
}
