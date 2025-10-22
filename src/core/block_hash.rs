use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct BlockHash(pub [u8; 32]); //  SHA-256 -> 32 bytes = 256 bits

impl Debug for BlockHash {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\"0х{}\"", hex::encode(self.0))
    }
}
