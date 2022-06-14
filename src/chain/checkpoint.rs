use std::hash::{Hash, Hasher};
use byte::ctx::{Bytes, Endian, Str};
use byte::{BytesExt, TryRead, TryWrite};
use dash_spv_primitives::consensus::encode::VarInt;
use dash_spv_primitives::crypto::byte_util::{Reversable, Zeroable};
use dash_spv_primitives::crypto::data_ops::Data;
use dash_spv_primitives::crypto::UInt256;
use dash_spv_primitives::hashes::hex::FromHex;
use crate::common::Block;
use crate::common::block::BLOCK_UNKNOWN_HEIGHT;

pub enum CheckpointParameter {
    None = 0,
    MerkleRoot = 1,
    MasternodeList = 1 << 2,
    // chainWorkSize is a multiple of 32 bytes
    ChainWorkSize = 1 << 4,
}

enum CheckpointOptions {
    None = 0,
    SaveMerkleRoot = 1,
}


#[derive(Clone)]
pub struct Checkpoint {
    pub height: u32,
    pub block_hash: UInt256,
    pub timestamp: u32,
    pub chain_work: UInt256,
    pub target: u32,
    pub merkle_root: UInt256,
    pub masternode_list_name: Option<String>,
}

impl Checkpoint {

    pub fn genesis_devnet_checkpoint() -> Checkpoint {
        Checkpoint {
            height: 0,
            block_hash: UInt256::from_hex("000008ca1832a4baf228eb1553c03d3a2c8e02399550dd6ea8d65cec3ef23d2e").unwrap().reversed(),
            timestamp: 1417713337,
            masternode_list_name: None,
            merkle_root: UInt256::MIN,
            chain_work: UInt256::from_hex("0200000000000000000000000000000000000000000000000000000000000000").unwrap(),
            target: 0x207fffffu32
        }
    }


    pub fn checkpoint_from_block(block: Block, options: u8) -> Checkpoint {
        assert_ne!(block.height, BLOCK_UNKNOWN_HEIGHT, "Block height must be known");
        let merkle_root = if options & CheckpointOptions::SaveMerkleRoot != 0 { block.merkle_root } else { UInt256::MIN };
        Checkpoint::new(block.height, block.block_hash, block.timestamp, block.target, merkle_root, block.chain_work, None)
    }

    pub fn new(height: u32, block_hash: UInt256, timestamp: u32, target: u32, merkle_root: UInt256, chain_work: UInt256, masternode_list_name: Option<String>) -> Checkpoint {
        Checkpoint {
            height,
            block_hash,
            timestamp,
            chain_work,
            target,
            merkle_root,
            masternode_list_name
        }
    }

    pub fn chain_work_size(&self) -> u8 {
        let mut chain_work_size = 8;
        //for (uint8_t i = 7; i != UINT8_MAX; i--)
        (0..8).rev().for_each(|i| {
            // self.chainWork.u32[i] == 0
            if self.chain_work.0.read_with::<u32>(&mut 0, byte::LE).unwrap() == 0 {
                chain_work_size -= 1;
            }
        });
        chain_work_size
    }

    pub fn parameters(&self) -> u8 {
        let mut parameters = 0;
        if !self.merkle_root.is_zero() {
            parameters |= CheckpointParameter::MerkleRoot;
        }
        if self.masternode_list_name.is_some() {
            parameters |= CheckpointParameter::MasternodeList;
        }
        parameters |= CheckpointParameter::ChainWorkSize * self.chain_work_size();
        parameters
    }

}

impl<'a> TryRead<'a, Endian> for Checkpoint {
    fn try_read(bytes: &'a [u8], _ctx: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let parameters = bytes.read_with::<u8>(offset, byte::LE)?;
        let height = bytes.read_with::<u32>(offset, byte::LE)?;
        let block_hash = bytes.read_with::<UInt256>(offset, byte::LE)?;
        let timestamp = bytes.read_with::<u32>(offset, byte::LE)?;
        let target = bytes.read_with::<u32>(offset, byte::LE)?;
        let chain_work_size = parameters >> 4;
        // let chain_work = UInt256::MIN;
        // (0..chain_work_size).into_iter().for_each(|i| {
            // let chain_work_section = bytes.read_with::<u32>(offset, byte::LE)?;
            //chain_work.u32[i] = chain_work_section;
        // });
        let chain_work_bytes = bytes.read_with(offset, Bytes::Len((4 * chain_work_size) as usize))?;

        let chain_work = UInt256(chain_work_bytes);
        let merkle_root = if parameters & DSCheckpointParameter_MerkleRoot != 0 {
            bytes.read_with::<UInt256>(offset, byte::LE)?
        } else {
            UInt256::MIN
        };

        let str_length = bytes.read_with::<VarInt>(offset, byte::LE)?.0 as usize;
        let masternode_list_name = matbytes.read_with::<&str>(offset, Str::Len(str_length)).unwrap_or(None);
        Ok((Checkpoint {
            height,
            block_hash,
            timestamp,
            chain_work,
            target,
            merkle_root,
            masternode_list_name
        }, *offset))
    }
}

impl TryWrite<Endian> for Checkpoint {
    fn try_write(self, bytes: &mut [u8], endian: Endian) -> byte::Result<usize> {
        let offset = &mut 0;
        bytes.write_with::<u8>(offset, self.parameters(), endian)?;
        bytes.write_with::<u32>(offset, self.height, endian)?;
        bytes.write_with::<UInt256>(offset, self.block_hash, endian)?;
        bytes.write_with::<u32>(offset, self.timestamp, endian)?;
        bytes.write_with::<u32>(offset, self.target, endian)?;
        (0..self.chain_work_size()).into_iter().for_each(|i| {
            let work = self.chain_work.0.read_with::<u32>(&mut 0, byte::LE).unwrap();
            bytes.write_with::<u32>(offset, work, endian)?;
        });
        if !self.merkle_root.is_zero() {
            bytes.write_with::<UInt256>(offset, self.merkle_root, endian)?;
        }
        if let Some(list_name) = self.masternode_list_name {
            bytes.write::<&str>(offset, list_name.as_str())?;
        }
        bytes.write_with::<u16>(offset, self.name.len() as u16, endian)?;
        bytes.write::<&str>(offset, self.name)?;
        bytes.write::<bool>(offset, self.enabled)?;
        Ok(*offset)
    }
}

impl Hash for Checkpoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut data: Vec<u8> = Vec::new();
        data.write_with::<Checkpoint>(&mut 0, (*self).clone(), byte::LE).unwrap();
        let hashed = data.to_sha256d();
        hashed.hash(state)
    }
}
