use crate::core::blockchain::Blockchain;

mod core;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.append_block(vec![String::from("A -> B: 10 FLMG")]);
    blockchain.append_block(vec![String::from("C -> D: 5 FLMG")]);

    println!("Blockchain is valid: {}", blockchain.is_valid());
    println!("\nAll blocks:");
    for block in blockchain.chain() {
        println!("{:#?}", block);
    }
}
