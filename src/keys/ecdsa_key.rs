use dash_spv_primitives::crypto::UInt256;
use crate::keys::key::IKey;

pub struct ECDSAKey {
    pub seckey: UInt256,
    pub pubkey: Option<UInt256>,
    pub compressed: bool,
    pub chaincode: Option<UInt256>,
    pub fingerprint: Option<u32>,
    pub is_extended: bool,
}

impl ECDSAKey {
    pub fn key_with_secret(secret: UInt256, compressed: bool) -> Self {
        Self {
            seckey: secret,
            pubkey: None,
            compressed,
            chaincode: None,
            fingerprint: None,
            is_extended: false
        }
    }
    pub fn key_with_extended_public_key_data(data: Vec<u8>) -> Self {
        Self {
            seckey: Default::default(),
            pubkey: None,
            compressed: false,
            chaincode: None,
            fingerprint: None,
            is_extended: false
        }
    }
}
impl IKey for ECDSAKey {

}
