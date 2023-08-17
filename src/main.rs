mod blockchain;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();

    let new_tx = blockchain::transaction::Transaction::new("1".to_string(), "2".to_string(), 1, 1);
    let new_block =
        blockchain::block::Block::new("1".to_string(), "1".to_string(), 1, vec![new_tx], 1);

    blockchain.add_block(new_block);
    println!("{:?}", blockchain);
}
