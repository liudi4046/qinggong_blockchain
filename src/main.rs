mod blockchain;
mod utils;
use blockchain::block::Block;
use blockchain::transaction::Transaction;
use utils::time;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();

    blockchain.add_genesis_block();
    for i in 1..=3 {
        let new_tx = Transaction::new("1".to_string(), "2".to_string(), 1, 1);
        let latest_block = blockchain.get_latest_block();
        let cur_time = time::current_timestamp();
        let mut new_block = Block::new(latest_block.pre_hash, cur_time, vec![new_tx], 5);
        let (hash, nonce) = new_block.cal_hash_and_nonce();
        new_block.update_block(hash, nonce);
        blockchain.add_block(new_block);
        println!("{:?}", blockchain);
    }
}
