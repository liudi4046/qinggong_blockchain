pub mod block;
pub mod transaction;

use super::utils::time;
use block::Block;
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
    pub fn validate_block(&mut self, new_block: Block) {}
    pub fn add_genesis_block(&mut self) {
        let cur_time = time::current_timestamp();
        let transaction = Transaction::new("0".to_string(), "123456".to_string(), 50, cur_time);
        let genesis_block = Block::new([0; 32], cur_time, vec![transaction], 4);
        self.blocks.push(genesis_block);
    }
    pub fn get_latest_block(&mut self) -> &Block {
        &self.blocks[self.blocks.len() - 1]
    }
}
