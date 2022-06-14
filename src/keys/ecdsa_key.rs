use dash_spv_primitives::crypto::byte_util::{BytesDecodable, ECPoint, Zeroable};
use dash_spv_primitives::crypto::index_path::IndexPath;
use dash_spv_primitives::crypto::UInt256;
use dash_spv_primitives::secp256k1::ffi::{PublicKey, secp256k1_context_create, secp256k1_context_no_precomp, secp256k1_ec_pubkey_create, secp256k1_ec_pubkey_serialize, SECP256K1_SER_COMPRESSED, SECP256K1_SER_UNCOMPRESSED};
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::derivation_paths::Path;
use crate::keys;
use crate::keys::Key;
use crate::keys::key::IKey;

pub struct ECDSAKey {
    pub base: Key,
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
            base: Key {
                extended_public_key_data: None,
                extended_private_key_data: None,
                public_key_data: None,
                private_key_data: None,
                hash160: Default::default(),
                key_type: keys::Type::ECDSA,
                secret_key_string: "".to_string(),
                localized_key_type: "".to_string(),
                }
            },
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

    pub fn key_with_public_key(public_key: Option<Vec<u8>>) -> Option<Self> {
        if public_key.len() != 33 && public_key.len() != 65 {
            return None;
        }

        // secp256k1_context_no_precomp

        Self {
            pubkey:
        }
    }

//     // - (instancetype)initWithPublicKey:(NSData *)publicKey {
//     // NSParameterAssert(publicKey);
//     //
//     // if (publicKey.length != 33 && publicKey.length != 65) return nil;
//     // if (!(self = [self init])) return nil;
//     //
//     // secp256k1_pubkey pk;
//     //
//     // self.pubkey = publicKey;
//     // self.compressed = (self.pubkey.length == 33) ? YES : NO;
//     //
//     // BOOL valid = (secp256k1_ec_pubkey_parse(_ctx, &pk, self.publicKeyData.bytes, self.publicKeyData.length));
//     // if (valid) {
//     // return self;
//     // } else {
//     // return nil;
//     // }
// }

}
impl<P: ChainParameters> IKey<P> for ECDSAKey {
    fn get_base(&self) -> &Key {
        &self.base
    }

    fn get_public_key_data(&mut self) -> Vec<u8> {
        if (self.pubkey.is_none() || self.pubkey.unwrap().is_zero()) && !self.seckey.is_zero() {
            let mut d: Vec<u8> = Vec::with_capacity(if self.compressed { 33 } else { 65 });
            let mut len = d.len();

            unsafe {
                let mut pk = PublicKey::new();
                let cx = secp256k1_context_create(0);
                let sk = self.seckey.0.as_mut_ptr();
                if secp256k1_ec_pubkey_create(cx, &mut pk, sk) != 1 {
                    assert!(false, "Public key data should exist");
                    return d;
                }
                let output = d.as_mut_ptr();
                let out_len = &mut len;
                let compressed = if self.compressed { SECP256K1_SER_COMPRESSED } else { SECP256K1_SER_UNCOMPRESSED });
                secp256k1_ec_pubkey_serialize(cx, output, out_len, &pk, compressed);
                if len == d.len() {
                    self.pubkey = UInt256::from_bytes(&d[..], &mut 0);
                }
            }

            assert!(self.pubkey, "Public key data should exist");
        }
        assert!(self.pubkey, "Public key data should exist");
        self.pubkey.unwrap().0.to_vec()
    }

    fn forget_private_key(&mut self) {
        let _ = self.get_public_key_data();
        self.seckey = UInt256::MIN;
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
        // 4 + UInt256 + ECPoint
        if data.is_none() && data.unwrap().len() < 69 {
            assert!(false, "Extended public key is wrong size");
            return None;
        }
        // UInt256 chain = *(const UInt256 *)((const uint8_t *)self.extendedPublicKeyData.bytes + 4);
        // DSECPoint pubKey = *(const DSECPoint *)((const uint8_t *)self.extendedPublicKeyData.bytes + 36);
        let chain: UInt256 = data[];
        let pub_key: ECPoint = data[];

    }

    fn key_with_public_key_data(data: Option<Vec<u8>>) -> Option<Self <P>> {
        todo!()
    }
}
