use crate::core::{BlockHash, Transaction};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    number: u64,                    // just a serial number of block
    transactions: Vec<Transaction>, // array of transactions
    previous_hash: BlockHash,       // SHA-256 of previous block
    hash: BlockHash,                // SHA-256 of current block
    timestamp: u64,                 // block creation timestamp
}

impl Block {
    pub fn new(
        block_number: u64,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
    ) -> Self {
        let mut block = Block {
            number: block_number,
            transactions,
            timestamp: get_timestamp(),
            previous_hash,
            hash: BlockHash([0; 32]),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn is_valid(&self) -> bool {
        let current_hash = self.calculate_hash();
        self.hash.0 == current_hash.0
    }

    fn calculate_hash(&self) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(self.number.to_le_bytes());
        hasher.update(&self.previous_hash.0);
        hasher.update(self.timestamp.to_le_bytes());

        for tx in &self.transactions {
            hasher.update(tx.as_bytes());
        }

        BlockHash(hasher.finalize().into())
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn previous_hash(&self) -> &BlockHash {
        &self.previous_hash
    }

    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    #[cfg(test)]
    pub fn tamper_transaction(&mut self, idx: usize, new_tx: String) {
        self.transactions[idx] = new_tx;
    }
}

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time should go forward")
        .as_secs()
}
