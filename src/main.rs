mod blockchain;
mod utils;
use blockchain::block::{self, Block};
use blockchain::transaction::Transaction;
use utils::time;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();

    blockchain.add_genesis_block();
    for i in 1..=10 {
        let new_tx = Transaction::new("1".to_string(), "2".to_string(), 1, 1);
        let pre_block = blockchain.get_block(i - 1);
        let cur_time = time::current_timestamp();
        let mut new_block = Block::new(pre_block.hash, cur_time, vec![new_tx], 3);
        let (hash, nonce) = new_block.cal_hash_and_nonce();
        new_block.update_block(hash, nonce);
        blockchain.add_block(new_block);
    }
    blockchain.print_blocks();
    for i in 1..=10 {
        let cur_block = blockchain.get_block(i);
        if blockchain.validate_block(cur_block, i) {
            println!("block#{} valid", i)
        } else {
            println!("block#{} invalid", i);
            break;
        }
    }
}
