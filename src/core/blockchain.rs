use crate::core::address::Address;
use crate::core::{Block, BlockHash, Transaction};
use std::collections::HashMap;

pub struct Blockchain {
    chain: Vec<Block>, // chain of blocks
}

impl Blockchain {
    pub fn new(initial_balances: Vec<(Address, u64)>) -> Self {
        let mut initial_state = HashMap::new();
        for (address, balance) in initial_balances {
            initial_state.insert(address, balance);
        }

        let master_block = Block::new(0, vec![], initial_state, BlockHash([0; 32]));
        println!("Master block: {:#?}", master_block);

        Self {
            chain: vec![master_block],
        }
    }

    pub fn append_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let previous_hash = previous_block.hash().clone();
        let block_number = previous_block.number() + 1;

        // clone full state
        let mut new_state = previous_block.state().clone();

        // apply all transactions to the state
        for tx in &transactions {
            let from_balance = new_state.get(&tx.from).unwrap_or(&0);
            if *from_balance < tx.amount {
                // will handle correctly later
                panic!("Insufficient balance!");
            }

            *new_state.get_mut(&tx.from).unwrap() -= tx.amount;
            *new_state.entry(tx.to.clone()).or_insert(0) += tx.amount;
        }

        let new_block = Block::new(block_number, transactions, new_state, previous_hash);
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
    fn test_blockchain_creation_with_state() {
        let blockchain = Blockchain::new(vec![(String::from("A"), 10), (String::from("B"), 5)]);
        assert_eq!(blockchain.chain.len(), 1);
        assert!(blockchain.is_valid());

        let genesis_block = &blockchain.chain.last().unwrap();
        assert_eq!(genesis_block.number(), 0);
        assert_eq!(*genesis_block.state().get("A").unwrap(), 10);
        assert_eq!(*genesis_block.state().get("B").unwrap(), 5);
    }

    #[test]
    fn test_valid_blockchain_and_state_after_transactions() {
        let mut blockchain = Blockchain::new(vec![(String::from("A"), 10), (String::from("C"), 5)]);
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

        assert_eq!(blockchain.chain.len(), 3);
        assert!(blockchain.is_valid());

        // initial: A=10, C=5
        // A -(10)-> B: A=0, B=10
        // C -(5)-> D: C=0, D=5
        let last_block = &blockchain.chain.last().unwrap();
        assert_eq!(last_block.number(), 2);
        assert_eq!(*last_block.state().get("A").unwrap(), 0);
        assert_eq!(*last_block.state().get("B").unwrap(), 10);
        assert_eq!(*last_block.state().get("C").unwrap(), 0);
        assert_eq!(*last_block.state().get("D").unwrap(), 5);
    }

    #[test]
    #[should_panic(expected = "Insufficient balance")]
    fn test_insufficient_balance_panics() {
        let mut blockchain = Blockchain::new(vec![(String::from("A"), 10)]);

        blockchain.append_block(vec![Transaction::new(
            String::from("A"),
            String::from("B"),
            15,
        )]);
    }

    #[test]
    fn test_tampered_transaction_blockchain_invalid() {
        let mut blockchain = Blockchain::new(vec![(String::from("A"), 10), (String::from("C"), 5)]);
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

        blockchain.chain[1]
            .tamper_transaction(0, Transaction::new(String::from("A"), String::from("B"), 2));

        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_tampered_state_blockchain_invalid() {
        let mut blockchain = Blockchain::new(vec![(String::from("A"), 10), (String::from("C"), 5)]);
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

        blockchain.chain[1].tamper_state(String::from("F"), 15);

        assert!(!blockchain.is_valid());
    }
}
