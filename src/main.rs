use crate::core::blockchain::Blockchain;
use crate::core::Transaction;

mod core;

fn main() {
    let mut blockchain = Blockchain::new(vec![(String::from("A"), 50), (String::from("C"), 50)]);
    blockchain.append_block(vec![Transaction::new(
        String::from("A"),
        String::from("B"),
        10,
    )]);
    blockchain.append_block(vec![Transaction::new(
        String::from("C"),
        String::from("D"),
        5,
    )]);

    println!("Blockchain is valid: {}", blockchain.is_valid());
    println!("\nAll blocks:");
    for block in blockchain.chain() {
        println!("{:#?}", block);
    }
}
