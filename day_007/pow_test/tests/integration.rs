use pow_test::{Block , mine_block ,compute_hash}

#[test]
fn test__full_mining(){
    let block = Block {
        id: 1,
        nonce: 0,
        data: String::from("integration_test"),
    };
    let nonce = mine_block(&block , 1).expect("Mining failed");
    let mut mined_block = block;
    mined_block.nonce = nonce;
    let hash = compute_hash(&mined_block);
    assert!(hash.starts_with("0"), "Hash does not meet difficulty");

}