use dash_spv_primitives::crypto::UInt256;
use crate::common::LLMQType;

#[derive(Debug, Copy, Clone)]
pub struct LLMQTypedHash {
    pub r#type: LLMQType,
    pub hash: UInt256,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LLMQIndexedHash {
    pub index: u32,
    pub hash: UInt256,
}

impl LLMQIndexedHash {
    pub fn new(hash: UInt256, index: u32) -> Self {
        LLMQIndexedHash { index, hash }
    }
}
