fn  validate_transaction(amount :u64)->bool {
    amount <=100
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validate_transaction(){
        assert!(validate_transaction(10));
        assert!(validate_transaction(50));

    }
}


// #[macro_use]
// extern  crate afl;

// fn process_data(data : &[u8]){
//     if let Ok(s)= std::str::from_utf8(data){
//         // Simulate the blockchain data processing
//         if s.len() <100 {
//             println!("Processed :{}",s);
//         }

    
//     }
// }
// fn main(){
//     fuzz
// }


fn main() {
    let block = Block {
        id: 1,
        nonce: 0,
        data: String::from("Hello Blockchain!"),
    };
    mine_block(&block, 3);
}
