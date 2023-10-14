use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;

pub mod blockchain;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                }],
        }],
        difficulty);
}

fn mining() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        difficulty,
    );
    block.mine();
    println!("Mined genesis block {:?}", &block);
    let mut last_hash = block.hash.clone();
    let mut blockchain = Blockchain { blocks: vec![block] };
    for i in 1..=10 {
        let mut block = Block::new(i, 0, last_hash, 0, "Another block!".to_owned(), difficulty);
        block.mine();
        println!("Mined block {:?}", &block);
        last_hash = block.hash.clone();
        blockchain.blocks.push(block);
    }
}
