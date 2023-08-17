pub mod block;
pub mod transaction;

use block::Block;
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
}
