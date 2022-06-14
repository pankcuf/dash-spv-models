use dash_spv_primitives::crypto::{UInt256, UInt384};
use dash_spv_primitives::crypto::index_path::IndexPath;
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::derivation_paths::Path;
use crate::keys;
use crate::keys::Key;
use crate::keys::key::IKey;

pub struct BLSKey {
    pub base: Key,
    pub public_key: UInt384,
    pub secret_key: UInt256,
    pub chain_code: UInt256,
}

impl IKey<P: ChainParameters> for BLSKey {
    fn get_base(&self) -> &Key {
        &self.base
    }

    fn get_public_key_data(&self) -> Vec<u8> {
        self.public_key.0.to_vec()
    }

    fn forget_private_key(&mut self) {
        self.secret_key = UInt256::MIN;
    }

    fn private_derive_to_path(&self, index_path: IndexPath<usize>) -> Option<Self> {
        todo!()
    }

    fn private_derive_to_256_bit_derivation_path(&self, derivation_path: Path<P>) -> Option<Self> {
        todo!()
    }

    fn serialized_private_key_for_chain(&self, chain: Chain<P>) -> Option<String> {
        todo!()
    }

    fn public_key_from_extended_public_key_data_at_index_path(data: Option<Vec<u8>>, index_path: IndexPath<usize>) -> Option<Vec<u8>> {
        // let extended_public_key = BLSKey::ke
        let extendedPublicKey = BLSKey::k [DSBLSKey keyWithExtendedPublicKeyData:self.extendedPublicKeyData];
        DSBLSKey *extendedPublicKeyAtIndexPath = [extendedPublicKey publicDeriveToPath:indexPath];
        NSData *data = [NSData dataWithUInt384:extendedPublicKeyAtIndexPath.publicKey];
        NSAssert(data, @"Public key should be created");
        return data;
    }

    fn key_with_public_key_data(data: Option<Vec<u8>>) -> Option<Self <P>> {
        todo!()
    }
}
