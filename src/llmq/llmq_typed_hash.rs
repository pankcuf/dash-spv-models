use dash_spv_primitives::crypto::UInt256;
use crate::common::LLMQType;

#[derive(Debug)]
pub struct LLMQTypedHash {
    pub r#type: LLMQType,
    pub hash: UInt256,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct LLMQIndexedHash {
    pub index: u32,
    pub hash: UInt256,
}
