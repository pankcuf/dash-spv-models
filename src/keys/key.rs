use dash_spv_primitives::crypto::index_path::IndexPath;
use dash_spv_primitives::crypto::UInt160;
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::derivation_paths::Path;
use crate::derivation_paths::path::DerivationPath;
use crate::keys;
use crate::keys::{BLSKey, ECDSAKey, Type};

pub trait IKey<P: ChainParameters> {
    fn get_base(&self) -> &Key;
    fn get_public_key_data(&self) -> Vec<u8>;

    fn forget_private_key(&mut self);
    fn private_derive_to_path(&self, index_path: IndexPath<usize>) -> Option<Self<P>>;
    fn private_derive_to_256_bit_derivation_path<DP: DerivationPath<P>>(&self, derivation_path: DP) -> Option<Self<P>>;
    fn serialized_private_key_for_chain(&self, chain: Chain<P>) -> Option<String>;
    fn public_key_from_extended_public_key_data_at_index_path(data: Option<Vec<u8>>, index_path: IndexPath<usize>) -> Option<Vec<u8>>;
    fn key_with_public_key_data(data: Option<Vec<u8>>) -> Option<Self<P>>;
}

pub struct Key {
    pub extended_public_key_data: Option<Vec<u8>>,
    pub extended_private_key_data: Option<Vec<u8>>,
    pub public_key_data: Option<Vec<u8>>,
    pub private_key_data: Option<Vec<u8>>,

    pub hash160: UInt160,
    pub key_type: Type,
    pub secret_key_string: String,
    pub localized_key_type: String,
}

impl Key {

    pub fn address_with_public_key_data(data: Option<Vec<u8>>, version: u8) -> Option<String> {
        assert!(data);
    }


    pub fn address_from_hash_160_data(data: Option<Vec<u8>>, version: u8) -> Option<String> {
        assert!(data);
    }

    pub fn key_with_seed_data<P: ChainParameters>(data: Option<Vec<u8>>, key_type: keys::Type) -> Option<dyn IKey<P>> {
        match key_type {
            Type::ECDSA => ECDSAKey::key_with_public_key(data),
            Type::BLS => BLSKey::key
        }
    }

    pub fn key_with_extended_private_key_data(data: Option<Vec<u8>>, key_type: Type) -> Option<dyn IKey<P>> {
        if data.is_none() {
            return None;
        }
        match key_type {
            Type::ECDSA => ECDSAKey::key_with_extended_private_key_data(data),
            Type::BLS => BLSKey::key_with_extended_private_key_data(data)
        }
    }


    + (NSString *)addressWithPublicKeyData:(NSData *)data forChain:(DSChain *)chain {
    NSParameterAssert(data);
    NSParameterAssert(chain);

    NSMutableData *d = [NSMutableData secureDataWithCapacity:160 / 8 + 1];
    uint8_t version;
    UInt160 hash160 = data.hash160;

    if ([chain isMainnet]) {
        version = DASH_PUBKEY_ADDRESS;
    } else {
        version = DASH_PUBKEY_ADDRESS_TEST;
    }

    [d appendBytes:&version length:1];
    [d appendBytes:&hash160 length:sizeof(hash160)];
    return [NSString base58checkWithData:d];
}

}
