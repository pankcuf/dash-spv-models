use byte::ctx::{Bytes, Endian};
use byte::{BytesExt, LE, TryRead};
use dash_spv_primitives::consensus::encode::VarInt;
use dash_spv_primitives::crypto::byte_util::BytesDecodable;
use dash_spv_primitives::hashes::hex::ToHex;
use dash_spv_primitives::impl_bytes_decodable;
use crate::common::LLMQSnapshotSkipMode;

#[derive(Clone)]
pub struct LLMQSnapshot {
    // The bitset of nodes already in quarters at the start of cycle at height n
    // (masternodeListSize + 7)/8
    pub member_list: Vec<u8>,
    // Skiplist at height n
    pub skip_list: Vec<u32>,
    //  Mode of the skip list
    pub skip_list_mode: LLMQSnapshotSkipMode,
}
impl Default for LLMQSnapshot {
    fn default() -> Self {
        Self {
            member_list: vec![],
            skip_list: vec![],
            skip_list_mode: LLMQSnapshotSkipMode::NoSkipping
        }
    }
}

impl<'a> std::fmt::Debug for LLMQSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LLMQSnapshot")
            .field("member_list", &self.member_list.to_hex())
            .field("skip_list", &self.skip_list.iter())
            .field("skip_list_mode", &self.skip_list_mode)
            .finish()
    }
}
impl<'a> TryRead<'a, Endian> for LLMQSnapshot {
    fn try_read(bytes: &'a [u8], _endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let skip_list_mode = bytes.read_with::<LLMQSnapshotSkipMode>(offset, LE)?;
        let member_list_length = bytes.read_with::<VarInt>(offset, LE)?.0 as usize;
        let member_list: &[u8] = bytes.read_with(offset, Bytes::Len((member_list_length + 7) / 8))?;
        let skip_list_length = bytes.read_with::<VarInt>(offset, LE)?.0 as usize;
        let mut skip_list = Vec::with_capacity(skip_list_length);
        for _i in 0..skip_list_length {
            skip_list.push(bytes.read_with::<u32>(offset, LE)?);
        }
        Ok((Self { member_list: member_list.to_vec(), skip_list, skip_list_mode }, *offset))
    }
}

impl LLMQSnapshot {
    pub fn length(&self) -> usize {
        self.member_list.len() + 1 + 2 + self.skip_list.len() * 2
    }
}
impl_bytes_decodable!(LLMQSnapshot);
