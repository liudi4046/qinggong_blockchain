use super::transaction::Transaction;
#[derive(Debug)]

pub struct Block {
    hash: String,
    pre_hash: String,
    timestamp: u64,
    transactions: Vec<Transaction>,
    nonce: u64,
}
impl Block {
    pub fn new(
        hash: String,
        pre_hash: String,
        timestamp: u64,
        transactions: Vec<Transaction>,
        nonce: u64,
    ) -> Block {
        Block {
            hash,
            pre_hash,
            timestamp,
            transactions,
            nonce,
        }
    }
}
