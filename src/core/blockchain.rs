use crate::core::{Block, BlockHash, Transaction};

pub struct Blockchain {
    chain: Vec<Block>, // chain of blocks
}

impl Blockchain {
    pub fn new() -> Self {
        let master_block = Block::new(0, vec![], BlockHash([0; 32]));
        println!("Master block: {:#?}", master_block);

        Blockchain {
            chain: vec![master_block],
        }
    }

    pub fn append_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let previous_hash = previous_block.hash().clone();
        let block_number = previous_block.number() + 1;

        let new_block = Block::new(block_number, transactions, previous_hash);
        println!("Appending block: {:#?}", new_block);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for block_index in 1..self.chain.len() {
            let current_block = &self.chain[block_index];
            let previous_block = &self.chain[block_index - 1];

            if !current_block.is_valid() {
                println!("Block {} has invalid hash", current_block.number());
                return false;
            }

            if current_block.previous_hash().0 != previous_block.hash().0 {
                println!("Block {} has invalid previous_hash", current_block.number());
                return false;
            }
        }
        true
    }

    pub fn chain(&self) -> &Vec<Block> {
        &self.chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1);
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_valid_blockchain() {
        let mut blockchain = Blockchain::new();
        blockchain.append_block(vec![String::from("A -> B: 10 FLMG")]);
        blockchain.append_block(vec![String::from("C -> D: 5 FLMG")]);

        assert_eq!(blockchain.chain.len(), 3);
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_tampered_blockchain_invalid() {
        let mut blockchain = Blockchain::new();
        blockchain.append_block(vec![String::from("A -> B: 10 FLMG")]);
        blockchain.append_block(vec![String::from("C -> D: 5 FLMG")]);

        blockchain.chain[1].tamper_transaction(0, String::from("A -> B: 1000 FLMG"));

        assert!(!blockchain.is_valid());
    }
}
