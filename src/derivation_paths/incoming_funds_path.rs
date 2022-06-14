use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use array_tool::vec::Intersect;
use byte::BytesExt;
use dash_spv_primitives::crypto::data_ops::{Data, short_hex_string_from, uint256_from_long};
use dash_spv_primitives::crypto::index_path::IndexPath;
use dash_spv_primitives::crypto::UInt256;
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::derivation_paths;
use crate::derivation_paths::{Path, Reference};
use crate::derivation_paths::funds_path::GapLimit;
use crate::derivation_paths::path::{BIP32_HARD, FEATURE_PURPOSE, FEATURE_PURPOSE_DASHPAY};
use crate::derivation_paths::simple_indexed_path::SimpleIndexedPath;
use crate::identity::identity::Identity;
use crate::keys::{ECDSAKey, IKey, Key, Type};

pub struct IncomingFundsPath<P: ChainParameters> {
    pub base: SimpleIndexedPath<P>,
    pub contact_source_blockchain_identity_unique_id: UInt256,
    pub contact_destination_blockchain_identity_unique_id: UInt256,
    pub external_derivation_path: bool,

    external_addresses: Vec<String>,
}

impl<P> IncomingFundsPath<P> {
    pub fn contact_based_derivation_path_with_destination_blockchain_identity_unique_id(
        destination_blockchain_identity_unique_id: UInt256,
        source_blockchain_identity_unique_id: UInt256,
        account_number: u32,
        chain: Chain<P>) -> Self<P> {
        assert_eq!(source_blockchain_identity_unique_id, destination_blockchain_identity_unique_id, "source and destination must be different");
        // TODO: full uint256 derivation
        IncomingFundsPath {
            base: SimpleIndexedPath {
                base: Path::init_with_indexes(
                    vec![
                        uint256_from_long(FEATURE_PURPOSE),
                        uint256_from_long(chain.chain_type.coin_type()),
                        uint256_from_long(FEATURE_PURPOSE_DASHPAY),
                        uint256_from_long(account_number as u64),
                        source_blockchain_identity_unique_id,
                        destination_blockchain_identity_unique_id
                    ],
                    vec![true, true, true, true, false, false],
                    derivation_paths::Type::ClearFunds,
                    Type::ECDSA,
                    Reference::ContactBasedFunds,
                    chain),
                ordered_addresses: vec![]
            },
            contact_source_blockchain_identity_unique_id: source_blockchain_identity_unique_id,
            contact_destination_blockchain_identity_unique_id: destination_blockchain_identity_unique_id,
            external_addresses: vec![],
            external_derivation_path: false
        }
    }

    // The extended public key will be saved to disk (store_external_derivation_path_extended_public_key_to_key_chain call needed)
    pub fn external_derivation_path_with_extended_public_key<T: IKey<P>>(extended_public_key: Option<T>, destination_blockchain_identity_unique_id: UInt256, source_blockchain_identity_unique_id: UInt256, chain: Chain<P>) -> Self<P> {
        // we are going to assume this is only ecdsa for now
        let mut base = Path::init_with_indexes(vec![], vec![], derivation_paths::Type::ViewOnlyFunds, Type::ECDSA, Reference::ContactBasedFundsExternal, chain);
        base.extended_public_key = extended_public_key;
        IncomingFundsPath {
            base: SimpleIndexedPath {
                base,
                ordered_addresses: vec![]
            },
            contact_source_blockchain_identity_unique_id: source_blockchain_identity_unique_id,
            contact_destination_blockchain_identity_unique_id: destination_blockchain_identity_unique_id,
            external_addresses: vec![],
            external_derivation_path: true
        }
    }

    // The extended public key will be loaded from disk
    pub fn external_derivation_path_with_extended_public_key_unique_id(extended_public_key_unique_id: String, destination_blockchain_identity_unique_id: UInt256, source_blockchain_identity_unique_id: UInt256, chain: Chain<P>) -> Self<P> {
        // we are going to assume this is only ecdsa for now
        let mut base = Path::init_with_indexes(vec![], vec![], derivation_paths::Type::ViewOnlyFunds, Type::ECDSA, Reference::ContactBasedFundsExternal, chain);
        base.standalone_extended_public_key_unique_id = Some(extended_public_key_unique_id);
        base.extended_public_key = extended_public_key;
        IncomingFundsPath {
            base: SimpleIndexedPath {
                base,
                ordered_addresses: vec![]
            },
            contact_source_blockchain_identity_unique_id: source_blockchain_identity_unique_id,
            contact_destination_blockchain_identity_unique_id: destination_blockchain_identity_unique_id,
            external_addresses: vec![],
            external_derivation_path: true
        }
    }

    // // returns the first unused external address
    //
    // @property (nonatomic, readonly, nullable) NSString *receiveAddress;
    //
    // // all previously generated external addresses
    // @property (nonatomic, readonly) NSArray *allReceiveAddresses;
    //
    // // used external addresses
    // @property (nonatomic, readonly) NSArray *usedReceiveAddresses;


    pub fn private_key_string_at_index(&self, n: usize, seed: Option<Vec<u8>>) -> Option<&String> {
        self.serialized_private_keys(vec![n], seed)?.last()
    }

    pub fn serialized_private_keys(&self, n: Vec<usize>, seed: Option<Vec<u8>>) -> Option<Vec<dyn IKey<P>>> {
        self.base.base.serialized_private_keys_at_index_paths(n.iter().map(|i|IndexPath::index_path_with_indexes(vec![i])).collect(), seed)
    }

    pub fn private_keys(&self, n: Vec<usize>, seed: Option<Vec<u8>>) -> Option<Vec<dyn IKey<P>>> {
        self.base.base.private_keys_at_index_paths(n.iter().map(|i| IndexPath::index_path_with_indexes(vec![i])).collect(), seed)
    }

    // gets a public key at an index
    pub fn public_key_data_at_index(&self, n: usize) -> Option<Vec<u8>> {
        self.base.base.public_key_data_at_index_path(IndexPath::index_path_with_indexes(vec![n]))
    }

    // gets an addess at an index one level down based on bip32
    pub fn address_at_index(&self, index: usize) -> Option<String> {
        // NSData *pubKey = [self publicKeyDataAtIndex:index];
        // return [[DSECDSAKey keyWithPublicKeyData:pubKey] addressForChain:self.chain];
        let pub_key = self.base.public_key_data_at_index(index);
        Key::address_with_public_key_data(pub_key, self.base.base.chain.chain_type.public_key_address())
    }

    pub fn receive_address_at_offset(&mut self, offset: usize) -> Option<&String> {
        // TODO: limit to 10,000 total addresses and utxos for practical usability with bloom filters
        if let Some(addrs) = self.register_addresses_with_gap_limit(offset + 1) {
            if let Some(addr) = addrs.last() {
                return Some(addr);
            }
        }
        self.external_addresses.last()
    }

    // all previously generated external addresses
    pub fn all_receive_addresses(&self) -> Vec<String> {
        self.external_addresses.clone()
    }

    pub fn used_receive_addresses(&self) -> Vec<String> {
        self.external_addresses.intersect(self.base.base.used_addresses.into_iter().collect())
    }


    pub fn index_path_for_known_address(&self, address: String) -> Option<IndexPath<usize>> {
        if let Some(index) = self.all_receive_addresses().iter().position(|&a| a == address) {
            Some(IndexPath::index_path_with_indexes(vec![index]))
        } else {
            None
        }
    }

    pub fn contact_source_blockchain_identity(&self) -> Option<&Identity> {
        self.base.base.chain.blockchain_identity_for_unique_id_in_wallet(self.contact_source_blockchain_identity_unique_id, None, true)
    }

    pub fn contact_destination_blockchain_identity(&self) -> Option<&Identity> {
        self.base.base.chain.blockchain_identity_for_unique_id_in_wallet(self.contact_destination_blockchain_identity_unique_id, None, true)
    }

    pub fn source_is_local(&self) -> bool {
        self.base.base.chain.blockchain_identity_for_unique_id(self.contact_source_blockchain_identity_unique_id).is_some()
    }

    pub fn destination_is_local(&self) -> bool {
        self.base.base.chain.blockchain_identity_for_unique_id(self.contact_destination_blockchain_identity_unique_id).is_some()
    }


    pub fn store_external_derivation_path_extended_public_key_to_key_chain(&self) {
        assert!(self.base.extended_public_key_data(), "the extended public key must exist");
        //setKeychainData(self.extendedPublicKeyData, self.standaloneExtendedPublicKeyLocationString, NO);
    }

    // inherited
    pub fn load_addresses(&mut self) {
        //[self loadAddressesInContext:self.managedObjectContext];
    }
    pub fn reload_addresses(&mut self) {
        self.external_addresses = vec![];
        self.base.used_addresses = HashSet::new();
        self.base.addresses_loaded = false;
        self.load_addresses();
    }

    pub fn account_number(&self) -> u64 {
        let i = self.base.index_at_position(self.base.indexes.len() - 3);
        let num = i.0.read_with::<u64>(&mut 0, byte::LE).unwrap();
        num & !BIP32_HARD
    }

    // returns the first unused external address
    pub fn receive_address(&mut self) -> Option<&String> {
        self.receive_address_in_context()
    }

    pub fn receive_address_in_context(&mut self) -> Option<&String> {
        self.receive_address_at_offset(0)
    }


    pub fn register_transaction_address(&mut self, address: String) -> bool {
        let has_addr = self.base.contains_address(Some(address));
        if has_addr {
            if !self.base.used_addresses.contains(&address) {
                self.base.used_addresses.insert(address.clone());
                self.register_addresses_with_gap_limit(GapLimit::Sequence.external());
            }
        }
        has_addr
    }

    pub fn create_identifier_for_derivation_path(&self) -> String {
        format!("{}-{}-{}",
                short_hex_string_from(&self.contact_source_blockchain_identity_unique_id.0),
                short_hex_string_from(&self.contact_destination_blockchain_identity_unique_id.0),
                short_hex_string_from(&self.base.extended_public_key_data().unwrap().to_sha256().0))
    }

    pub fn register_addresses_with_gap_limit(&mut self, gap_limit: usize) -> Option<Vec<String>> {
        //self.register_addresses_with_gap_limit(gap_limit)

        assert!(self.base.account, "Account must be set");
        if !self.account.wallet.isTransient {
            if !self.base.addresses_loaded {
                sleep(Duration::from_secs(1)); // quite hacky, we need to fix this
            }
            assert!(self.base.addresses_loaded, "addresses must be loaded before calling this function");
        }


        let mut arr = self.external_addresses.clone();
        let mut i = arr.len();

        // keep only the trailing contiguous block of addresses that aren't used
        while i > 0 && !self.base.used_addresses.contains(&arr[i - 1]) {
            i -= 1;
        }

        if i > 0 {
            arr.drain(0..i);
        }
        if arr.len() >= gap_limit {
            return Some(arr[0..gap_limit].to_owned());
        }

        if gap_limit > 1 { // get receive_address and changeAddress first to avoid blocking
            self.receive_address_in_context();
        }

        // @synchronized(self) {
        // It seems weird to repeat this, but it's correct because of the original call receive address and change address
        arr = self.external_addresses.clone();
        i = arr.len();
        let mut n = i;

        // keep only the trailing contiguous block of addresses with no transactions
        while i > 0 && !self.base.used_addresses.contains(&arr[i - 1]) {
            i -= 1;
        }
        if i > 0 {
            arr.drain(0..i);
        }
        if arr.len() >= gap_limit {
            return Some(arr[0..gap_limit].to_owned());
        }
        let mut upper_limit = gap_limit;

        while arr.len() < upper_limit { // generate new addresses up to gapLimit
            let pub_key_data = self.base.public_key_data_at_index(n);
            if let Some(address) = Key::address_with_public_key_data(pub_key_data, self.base.base.chain.chain_type.public_key_address()) {
                let mut is_used = false;
                // if let Some(account) = self.base.base.account {
                    // if let Some(wallet) = account.wallet {
                        // if !wallet.is_transient {
                            //     [context performBlockAndWait:^{ // store new address in core data
                            //         DSDerivationPathEntity *derivationPathEntity = [DSDerivationPathEntity derivationPathEntityMatchingDerivationPath:self inContext:context];
                            //         DSAddressEntity *e = [DSAddressEntity managedObjectInContext:context];
                            //         e.derivationPath = derivationPathEntity;
                            //         NSAssert([address isValidDashAddressOnChain:self.chain], @"the address is being saved to the wrong derivation path");
                            //         e.address = address;
                            //         e.index = n;
                            //         e.internal = NO;
                            //         e.standalone = NO;
                            //         NSArray *outputs = [DSTxOutputEntity objectsInContext:context matching:@"address == %@", address];
                            //         [e addUsedInOutputs:[NSSet setWithArray:outputs]];
                            //         if (outputs.count) isUsed = TRUE;
                            //     }];
                        // }
                    // }
                // }

                if is_used {
                    self.base.base.used_addresses.insert(address.clone());
                    upper_limit += 1;
                }
                self.base.base.all_addresses.insert(address.clone());
                self.external_addresses.push(address.clone());
                arr.push(address.clone());
                n += 1;

            } else {
                println!("error generating keys");
                // if (error) {
                //     *error = [NSError errorWithDomain:@"DashSync"
                //     code:500
                //     userInfo:@{NSLocalizedDescriptionKey:
                //         DSLocalizedString(@"Error generating public keys", nil)}];
                // }
                return None;
            }
        }
        Some(arr)
    // }
    }




}
