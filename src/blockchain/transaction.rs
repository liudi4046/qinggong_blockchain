#[derive(Debug)]

pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u32,
    timestamp: u64,
}
impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u32, timestamp: u64) -> Transaction {
        Transaction {
            sender,
            recipient,
            amount,
            timestamp,
        }
    }
}
