pub mod block;
pub mod transaction;

use super::utils::time;
use block::Block;
use sha2::{Digest, Sha256};
use transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }
    pub fn add_block(&mut self, new_block: Block) {
        //verification to be done
        self.blocks.push(new_block);
    }
    pub fn receive_block(&mut self, new_block: Block) {}
    pub fn validate_block(&self, block: &Block, index: usize) -> bool {
        let pre_block = &self.blocks[index - 1];

        if block.pre_hash != pre_block.hash {
            println!(
                "block.pre_hash: {:?} , pre_block.hash : {:?}",
                block.pre_hash, pre_block.hash
            );
            return false;
        }
        let mut hasher = Sha256::new();
        let pre_hash = hex::encode(block.pre_hash);
        let hash_input = format!(
            "{}{}{:?}{}{}",
            pre_hash, block.timestamp, block.transactions, block.difficulty, block.nonce
        );
        hasher.update(hash_input);
        let result: [u8; 32] = hasher.finalize().try_into().expect("error 4");
        println!(
            "current hash : {:?} , calculated hash : {:?}",
            block.hash, result
        );
        if result == block.hash {
            return true;
        }

        return false;
    }
    pub fn add_genesis_block(&mut self) {
        let cur_time = time::current_timestamp();
        let transaction = Transaction::new("0".to_string(), "123456".to_string(), 50, cur_time);
        let genesis_block = Block::new([0; 32], cur_time, vec![transaction], 4);

        self.blocks.push(genesis_block);
    }
    pub fn get_latest_block(&mut self) -> &Block {
        &self.blocks[self.blocks.len() - 1]
    }
    pub fn get_block(&self, index: usize) -> &Block {
        &self.blocks[index]
    }
    pub fn print_blocks(&self) {
        for block in &self.blocks {
            println!("{:?}\n", block);
        }
    }
}
