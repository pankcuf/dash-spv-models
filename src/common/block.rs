use dash_spv_primitives::crypto::UInt256;

pub const BLOCK_UNKNOWN_HEIGHT: i32 = i32::MAX;
pub const DGW_PAST_BLOCKS_MIN: i32 = 24;
pub const DGW_PAST_BLOCKS_MAX: i32 = 24;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub struct Block {
    pub height: u32,
    pub block_hash: UInt256,
    pub timestamp: u32,

}
