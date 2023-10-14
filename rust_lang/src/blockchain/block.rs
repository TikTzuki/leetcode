#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec,
    pub difficulty: u128,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }
}

// impl Debug for Block {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
//                &self.index,
//                &hex::encode(&self.hash),
//                &self.timestamp,
//                &self.transactions.len(),
//                &self.nonce, )
//     }
// }