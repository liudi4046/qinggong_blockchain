use super::transaction::Transaction;
use sha2::{Digest, Sha256};
#[derive(Debug)]

pub struct Block {
    pub hash: [u8; 32],
    pub pre_hash: [u8; 32],
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u64,
    pub nonce: u64,
}
impl Block {
    pub fn new(
        pre_hash: [u8; 32],
        timestamp: u64,
        transactions: Vec<Transaction>,
        difficulty: u64,
    ) -> Block {
        Block {
            hash: [0; 32],
            pre_hash,
            timestamp,
            transactions,
            difficulty,
            nonce: 0,
        }
    }

    pub fn cal_hash_and_nonce(&mut self) -> ([u8; 32], u64) {
        let mut nonce = 0;
        loop {
            let mut hasher = Sha256::new();
            let pre_hash = hex::encode(self.pre_hash);
            let hash_input = format!(
                "{}{}{:?}{}{}",
                pre_hash, self.timestamp, self.transactions, self.difficulty, nonce
            );
            hasher.update(hash_input);
            let result: [u8; 32] = hasher.finalize().try_into().expect("error 3");
            let hex_representation = hex::encode(result);
            if hex_representation.starts_with(&"0".repeat(self.difficulty.try_into().unwrap())) {
                return (result, nonce);
            }
            nonce += 1;
        }
    }

    pub fn update_block(&mut self, hash: [u8; 32], nonce: u64) {
        self.hash = hash;
        self.nonce = nonce;
    }
}
