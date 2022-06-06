use dash_spv_primitives::crypto::UInt160;
use crate::keys;

pub trait IKey {

}

pub struct Key<'a> {
    pub extended_public_key_data: Option<&'a [u8]>,
    pub extended_private_key_data: Option<&'a [u8]>,
    pub public_key_data: Option<&'a [u8]>,
    pub private_key_data: Option<&'a [u8]>,

    pub hash160: UInt160,
    pub key_type: keys::Type,
    pub secret_key_string: &'a str,
    pub localized_key_type: &'a str,
}

impl Key {
    // pub fn keyWithExtendedPublicKeyData() -> Self {
    //
    // }
}

impl<'a> IKey for Key<'a> {

}
