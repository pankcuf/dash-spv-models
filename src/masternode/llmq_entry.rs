use std::convert::Into;
use byte::{BytesExt, LE, TryRead};
use byte::ctx::{Bytes, Endian};
use dash_spv_primitives::consensus::{Encodable, WriteExt};
use dash_spv_primitives::consensus::encode::VarInt;
use dash_spv_primitives::crypto::{UInt256, UInt384, UInt768};
use dash_spv_primitives::crypto::data_ops::Data;
use dash_spv_primitives::hashes::{Hash, sha256d};
use dash_spv_primitives::hashes::hex::ToHex;
use crate::common::LLMQType;

pub const LLMQ_DEFAULT_VERSION: u16 = 1;
pub const LLMQ_INDEXED_VERSION: u16 = 2;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct LLMQEntry {
    pub version: u16,
    pub llmq_hash: UInt256,
    pub index: Option<u16>,
    pub public_key: UInt384,
    pub threshold_signature: UInt768,
    pub verification_vector_hash: UInt256,
    pub all_commitment_aggregated_signature: UInt768,
    pub llmq_type: LLMQType,
    pub signers_bitset: Vec<u8>,
    pub signers_count: VarInt,
    pub valid_members_bitset: Vec<u8>,
    pub valid_members_count: VarInt,
    pub entry_hash: UInt256,
    pub verified: bool,
    pub saved: bool,
    pub commitment_hash: Option<UInt256>,
}
impl std::fmt::Debug for LLMQEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LLMQEntry")
            .field("version", &self.version)
            // .field("llmq_hash", &self.llmq_hash)
            .field("index", &self.index.unwrap_or(0))
            // .field("public_key", &self.public_key)
            // .field("threshold_signature", &self.threshold_signature)
            // .field("verification_vector_hash", &self.verification_vector_hash)
            // .field("all_commitment_aggregated_signature", &self.all_commitment_aggregated_signature)
            .field("llmq_type", &self.llmq_type)
            // .field("signers_bitset", &self.signers_bitset.to_hex())
            // .field("signers_bitset_length", &self.signers_bitset.len())
            // .field("signers_count", &self.signers_count)
            // .field("valid_members_bitset", &self.valid_members_bitset.to_hex())
            // .field("valid_members_bitset_length", &self.valid_members_bitset.len())
            // .field("valid_members_count", &self.valid_members_count)
            .field("entry_hash", &self.entry_hash)
            .field("verified", &self.verified)
            // .field("commitment_hash", &self.commitment_hash)
            .finish()
    }
}

impl<'a> TryRead<'a, Endian> for LLMQEntry {
    fn try_read(bytes: &'a [u8], _ctx: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let version = bytes.read_with::<u16>(offset, LE)?;
        let llmq_type = bytes.read_with::<LLMQType>(offset, LE)?;
        let llmq_hash = bytes.read_with::<UInt256>(offset, LE)?;
        let index = if version >= LLMQ_INDEXED_VERSION {
            Some(bytes.read_with::<u16>(offset, LE)?)
        } else {
            None
        };
        let signers_count = bytes.read_with::<VarInt>(offset, LE)?;
        let signers_buffer_length: usize = ((signers_count.0 as usize) + 7) / 8;
        let signers_bitset = bytes.read_with(offset, Bytes::Len(signers_buffer_length))?;
        let valid_members_count = bytes.read_with::<VarInt>(offset, LE)?;
        let valid_members_count_buffer_length: usize = ((valid_members_count.0 as usize) + 7) / 8;
        let valid_members_bitset = bytes.read_with(offset, Bytes::Len(valid_members_count_buffer_length))?;
        let public_key = bytes.read_with::<UInt384>(offset, LE)?;
        let verification_vector_hash = bytes.read_with::<UInt256>(offset, LE)?;
        let threshold_signature = bytes.read_with::<UInt768>(offset, LE)?;
        let all_commitment_aggregated_signature = bytes.read_with::<UInt768>(offset, LE)?;
        let q_data = Self::generate_data(
            version, llmq_type, llmq_hash, index,
            signers_count.clone(), signers_bitset,
            valid_members_count.clone(), valid_members_bitset,
            public_key, verification_vector_hash, threshold_signature,
            all_commitment_aggregated_signature);
        let entry_hash = UInt256(sha256d::Hash::hash(q_data.as_slice()).into_inner());

        Ok((LLMQEntry {
            version,
            llmq_hash,
            index,
            public_key,
            threshold_signature,
            verification_vector_hash,
            all_commitment_aggregated_signature,
            signers_count,
            llmq_type,
            valid_members_count,
            signers_bitset: signers_bitset.to_vec(),
            valid_members_bitset: valid_members_bitset.to_vec(),
            entry_hash,
            verified: false,
            saved: false,
            commitment_hash: None
        }, *offset))
    }
}

impl LLMQEntry {

    pub fn new(version: u16, llmq_type: LLMQType, llmq_hash: UInt256, index: Option<u16>,
               signers_count: VarInt, valid_members_count: VarInt,
               signers_bitset: Vec<u8>, valid_members_bitset: Vec<u8>,
               public_key: UInt384, verification_vector_hash: UInt256,
               threshold_signature: UInt768, all_commitment_aggregated_signature: UInt768
    ) -> Self {
        let q_data = Self::generate_data(
            version, llmq_type, llmq_hash, index,
            signers_count.clone(), signers_bitset.as_slice(),
            valid_members_count.clone(), valid_members_bitset.as_slice(),
            public_key, verification_vector_hash, threshold_signature,
            all_commitment_aggregated_signature);
        let entry_hash = UInt256(sha256d::Hash::hash(q_data.as_slice()).into_inner());
        Self {
            version,
            llmq_hash,
            index,
            public_key,
            threshold_signature,
            verification_vector_hash,
            all_commitment_aggregated_signature,
            llmq_type,
            signers_bitset,
            signers_count,
            valid_members_bitset,
            valid_members_count,
            entry_hash,
            verified: false,
            saved: false,
            commitment_hash: None
        }
    }

    pub fn generate_data(
        version: u16,
        llmq_type: LLMQType,
        llmq_hash: UInt256,
        llmq_index: Option<u16>,
        signers_count: VarInt,
        signers_bitset: &[u8],
        valid_members_count: VarInt,
        valid_members_bitset: &[u8],
        public_key: UInt384,
        verification_vector_hash: UInt256,
        threshold_signature: UInt768,
        all_commitment_aggregated_signature: UInt768
    ) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let offset: &mut usize = &mut 0;
        let llmq_u8: u8 = llmq_type.into();
        *offset += version.consensus_encode(&mut buffer).unwrap();
        *offset += llmq_u8.consensus_encode(&mut buffer).unwrap();
        *offset += llmq_hash.consensus_encode(&mut buffer).unwrap();
        if let Some(index) = llmq_index {
            *offset += index.consensus_encode(&mut buffer).unwrap();
        }
        *offset += signers_count.consensus_encode(&mut buffer).unwrap();
        buffer.emit_slice(&signers_bitset).unwrap();
        *offset += signers_bitset.len();
        *offset += valid_members_count.consensus_encode(&mut buffer).unwrap();
        buffer.emit_slice(&valid_members_bitset).unwrap();
        *offset += valid_members_bitset.len();
        *offset += public_key.consensus_encode(&mut buffer).unwrap();
        *offset += verification_vector_hash.consensus_encode(&mut buffer).unwrap();
        *offset += threshold_signature.consensus_encode(&mut buffer).unwrap();
        *offset += all_commitment_aggregated_signature.consensus_encode(&mut buffer).unwrap();
        buffer
    }

    pub fn to_data(&self) -> Vec<u8> {
        Self::generate_data(
            self.version, self.llmq_type, self.llmq_hash, self.index,
            self.signers_count, &self.signers_bitset,
            self.valid_members_count, &self.valid_members_bitset,
            self.public_key, self.verification_vector_hash,
            self.threshold_signature, self.all_commitment_aggregated_signature)
    }

    pub fn llmq_quorum_hash(&self) -> UInt256 {
        let mut buffer: Vec<u8> = Vec::with_capacity(33);
        let offset: &mut usize = &mut 0;
        *offset += VarInt(self.llmq_type as u64).consensus_encode(&mut buffer).unwrap();
        *offset += self.llmq_hash.consensus_encode(&mut buffer).unwrap();
        UInt256(sha256d::Hash::hash(&buffer).into_inner())
    }

    pub fn commitment_data(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let offset: &mut usize = &mut 0;
        let llmq_type = VarInt(self.llmq_type as u64);
        *offset += llmq_type.consensus_encode(&mut buffer).unwrap();
        *offset += self.llmq_hash.consensus_encode(&mut buffer).unwrap();
        *offset += self.valid_members_count.consensus_encode(&mut buffer).unwrap();
        buffer.emit_slice(&self.valid_members_bitset).unwrap();
        *offset += self.valid_members_bitset.len();
        *offset += self.public_key.consensus_encode(&mut buffer).unwrap();
        *offset += self.verification_vector_hash.consensus_encode(&mut buffer).unwrap();
        buffer
    }

    pub fn ordering_hash_for_request_id(&self, request_id: UInt256, llmq_type: LLMQType) -> UInt256 {
        let mut buffer: Vec<u8> = Vec::new();
        let offset: &mut usize = &mut 0;
        let llmq_type = VarInt(llmq_type as u64);
        *offset += llmq_type.consensus_encode(&mut buffer).unwrap();
        *offset += self.llmq_hash.consensus_encode(&mut buffer).unwrap();
        *offset += request_id.consensus_encode(&mut buffer).unwrap();
        UInt256(sha256d::Hash::hash(&buffer).into_inner())
    }

    pub fn generate_commitment_hash(&mut self) -> UInt256 {
        if self.commitment_hash.is_none() {
            let data = self.commitment_data();
            self.commitment_hash = Some(UInt256(sha256d::Hash::hash(&data).into_inner()));
        }
        self.commitment_hash.unwrap()
    }

    pub fn validate_payload(&self) -> bool {
        // The quorumHash must match the current DKG session
        // todo
        // The byte size of the signers and validMembers bitvectors must match “(quorumSize + 7) / 8”
        if self.signers_bitset.len() != (self.signers_count.0 as usize + 7) / 8 {
            println!("Error: The byte size of the signers bitvectors ({}) must match “(quorumSize + 7) / 8 ({})“", self.signers_bitset.len(), (self.signers_count.0 + 7) / 8);
            return false;
        }
        if self.valid_members_bitset.len() != (self.valid_members_count.0 as usize + 7) / 8 {
            println!("Error: The byte size of the validMembers bitvectors ({}) must match “(quorumSize + 7) / 8 ({})", self.valid_members_bitset.len(), (self.valid_members_count.0 + 7) / 8);
            return false;
        }
        let signers_offset = (self.signers_count.0 / 8) as i32;
        let mut s_offset = signers_offset.clone() as usize;
        let signers_last_byte = self.signers_bitset.read_with::<u8>(&mut s_offset, LE).unwrap_or(0) as i32;
        let signers_mask = 255 >> (((8 - signers_offset) % 32) + 32) % 32 << (((8 - signers_offset) % 32) + 32) % 32;
        let signers_byte_and_mask = signers_last_byte & signers_mask;
        if signers_byte_and_mask != 0 {
            println!("Error: No out-of-range bits should be set in byte representation of the signers bitvector: {:?} {} {} {} {}", self.signers_bitset.to_hex(), self.signers_count, signers_last_byte, signers_mask, signers_byte_and_mask);
            return false;
        }
        let valid_members_offset = (self.valid_members_count.0 / 8) as i32;
        let mut v_offset = valid_members_offset.clone() as usize;
        let valid_members_last_byte = self.valid_members_bitset.read_with::<u8>(&mut v_offset, LE).unwrap_or(0) as i32;
        let valid_members_mask = 255 >> (((8 - valid_members_offset) % 32) + 32) % 32 << (((8 - valid_members_offset) % 32) + 32) % 32;
        let valid_members_byte_and_mask = valid_members_last_byte & valid_members_mask;
        if valid_members_byte_and_mask != 0 {
            println!("Error: No out-of-range bits should be set in byte representation of the validMembers bitvector: {:?} {} {} {} {}", self.valid_members_bitset.to_hex(), self.valid_members_count, valid_members_last_byte, valid_members_mask, valid_members_byte_and_mask);
            return false;
        }
        let quorum_threshold = self.llmq_type.threshold() as u64;
        // The number of set bits in the signers and validMembers bitvectors must be at least >= quorumThreshold
        let signers_bitset_true_bits_count = self.signers_bitset.true_bits_count();
        if signers_bitset_true_bits_count < quorum_threshold {
            println!("Error: The number of set bits in the signers bitvector {} must be at least >= quorumThreshold {}", signers_bitset_true_bits_count, quorum_threshold);
            return false;
        }
        let valid_members_bitset_true_bits_count = self.valid_members_bitset.true_bits_count();
        if valid_members_bitset_true_bits_count < quorum_threshold {
            println!("Error: The number of set bits in the validMembers bitvector {} must be at least >= quorumThreshold {}", valid_members_bitset_true_bits_count, quorum_threshold);
            return false;
        }
        true
    }
}
