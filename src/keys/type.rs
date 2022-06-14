use dash_spv_primitives::crypto::index_path::IndexPath;
use crate::chain::chain_parameters::ChainParameters;
use crate::keys::{BLSKey, ECDSAKey, IKey};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Type {
    ECDSA = 0,
    BLS = 1,
}

impl Type {
    pub fn public_key_data_at_index_path<P: ChainParameters>(&self, extended_public_key_data: Option<Vec<u8>>, index_path: IndexPath<usize>) -> Option<dyn IKey<P>> {
        match self {
            Type::ECDSA => ECDSAKey::public_key_from_extended_public_key_data_at_index_path(extended_public_key_data, index_path),
            Type::BLS => BLSKey::public_key_from_extended_public_key_data_at_index_path(extended_public_key_data, index_path)
        }
    }
}
