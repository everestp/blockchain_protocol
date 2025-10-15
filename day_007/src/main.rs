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