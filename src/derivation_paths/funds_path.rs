use std::collections::HashMap;
use std::ops::Range;
use array_tool::vec::Intersect;
use dash_spv_primitives::crypto::data_ops::uint256_from_long;
use dash_spv_primitives::crypto::index_path::IndexPath;
use dash_spv_primitives::crypto::UInt256;
use dash_spv_primitives::hashes::sha1::Hash;
use crate::chain::chain::Chain;
use crate::{derivation_paths, keys};
use crate::chain::chain_parameters::ChainParameters;
use crate::common::ChainType;
use crate::derivation_paths::{Path, Reference};
use crate::derivation_paths::path::DerivationPath;
use crate::keys::{ECDSAKey, Key};

pub const EXTENDED_0_PUBKEY_KEY_BIP44_V0: &str = "masterpubkeyBIP44"; // these are old and need to be retired
pub const EXTENDED_0_PUBKEY_KEY_BIP32_V0: &str = "masterpubkeyBIP32"; // these are old and need to be retired
pub const EXTENDED_0_PUBKEY_KEY_BIP44_V1: &str = "extended0pubkeyBIP44";
pub const EXTENDED_0_PUBKEY_KEY_BIP32_V1: &str = "extended0pubkeyBIP32";

pub const DERIVATION_PATH_IS_USED_KEY: &str = "DERIVATION_PATH_IS_USED_KEY";

pub enum GapLimit {
    Sequence,
    SequenceUnused,
    SequenceDashpay,
}

impl GapLimit {
    pub fn external(&self) -> usize {
        match self {
            GapLimit::Sequence => 10,
            GapLimit::SequenceUnused => 10,
            GapLimit::SequenceDashpay => 6,
        }
    }
    pub fn internal(&self) -> usize {
        match self {
            GapLimit::Sequence => 5,
            GapLimit::SequenceUnused => 5,
            GapLimit::SequenceDashpay => 3,
        }
    }
    pub fn initial(&self) -> usize {
        match self {
            GapLimit::Sequence => 100,
            GapLimit::SequenceUnused => 15,
            GapLimit::SequenceDashpay => 10,
        }
    }
}

pub struct FundsPath<P: ChainParameters> {
    pub base: Path<P>,
    // used external addresses
    pub used_receive_addresses: Vec<String>,
    // used internal addresses
    pub used_change_addresses: Vec<String>,

    internal_addresses: Vec<String>,
    external_addresses: Vec<String>,
    is_for_first_account: bool,
    has_known_balance_internal: bool,
    checked_initial_has_known_balance: bool,

}

impl<P: ChainParameters> FundsPath<P> {
    pub fn bip32_derivation_path_for_account_number(account_number: u32, chain: Chain<P>) -> FundsPath<P> {
        let indexes = vec![uint256_from_long(account_number as u64)];
        let hardened_indexes = vec![true];
        funds_path_for(account_number == 0, indexes, hardened_indexes, Reference::BIP32, chain)
    }

    pub fn bip44_derivation_path_for_account_number(account_number: u32, chain: Chain<P>) -> FundsPath<P> {
        let indexes = vec![uint256_from_long(44), uint256_from_long(chain.r#type.coin_type()), uint256_from_long(account_number as u64)];
        let hardened_indexes = vec![true, true, true];
        funds_path_for(account_number == 0, indexes, hardened_indexes, Reference::BIP44, chain)
    }

    fn funds_path_for(is_for_first_account: bool, indexes: Vec<UInt256>, hardened_indexes: Vec<bool>, reference: Reference, chain: Chain<P>) -> FundsPath<P> {
        FundsPath {
            base: Path::derivation_path_with_indexes(
                indexes,
                hardened_indexes,
                derivation_paths::Type::ClearFunds,
                keys::Type::ECDSA,
                reference,
                chain),
            used_receive_addresses: vec![],
            used_change_addresses: vec![],
            internal_addresses: vec![],
            external_addresses: vec![],
            is_for_first_account,
            has_known_balance_internal: false,
            checked_initial_has_known_balance: false
        }
    }

    // we should use a reduced gap limit on derivation paths with no balance (except account 0 on bip44)
    pub fn should_use_reduced_gap_limit(&self) -> bool {
        if !self.checked_initial_has_known_balance {
            // NSError *error = nil;
            // let hasKnownBalance = getKeychainInt([self hasKnownBalanceUniqueIDString], &error);
            // if (!error) {
            //     self.has_known_balance_internal = hasKnownBalance ? TRUE : FALSE;
            //     self.checked_initial_has_known_balance = true;
            // }
        }
        !self.has_known_balance_internal && !(self.is_for_first_account && self.base.reference == Reference::BIP44)
    }

    pub fn set_has_known_balance(&mut self) {
        if !self.has_known_balance_internal {
            //setKeychainInt(1, [self hasKnownBalanceUniqueIDString], NO);
            self.has_known_balance_internal = true;
        }
    }

    pub fn has_known_balance_unique_idstring(&self) -> String {
        format!("{}_{}_{:?}", DERIVATION_PATH_IS_USED_KEY, self.base.account.unwrap().unique_id, self.base.reference)
    }

    fn reload_addresses(&mut self) {
        self.internal_addresses = vec![];
        self.external_addresses = vec![];
        self.base.used_addresses.clear();
        self.base.addresses_loaded = false;
        self.load_addresses();
    }

    fn load_addresses(&mut self) {
        if !self.base.addresses_loaded {
            // [self.managedObjectContext performBlockAndWait:^{
            //     DSDerivationPathEntity *derivationPathEntity = [DSDerivationPathEntity derivationPathEntityMatchingDerivationPath:self inContext:self.managedObjectContext];
            //     self.syncBlockHeight = derivationPathEntity.syncBlockHeight;
            //     for (DSAddressEntity *e in derivationPathEntity.addresses) {
            //         @autoreleasepool {
            //             NSMutableArray *a = (e.internal) ? self.internalAddresses : self.externalAddresses;
            //
            //             while (e.index >= a.count) [a addObject:[NSNull null]];
            //             if (![e.address isValidDashAddressOnChain:self.account.wallet.chain]) {
            //             #if DEBUG
            //             DSLogPrivate(@"address %@ loaded but was not valid on chain %@", e.address, self.account.wallet.chain.name);
            //             #else
            //             DSLog(@"address %@ loaded but was not valid on chain %@", @"<REDACTED>", self.account.wallet.chain.name);
            //             #endif /* DEBUG */
            //             continue;
            //             }
            //             a[e.index] = e.address;
            //             [self.mAllAddresses addObject:e.address];
            //             if ([e.usedInInputs count] || [e.usedInOutputs count]) {
            //             [self.mUsedAddresses addObject:e.address];
            //             }
            //         }
            //     }
            // }];
            self.base.addresses_loaded = true;
            let gap_limit = if self.should_use_reduced_gap_limit() {
                GapLimit::SequenceUnused
            } else {
                GapLimit::Sequence
            }.initial();
            self.register_addresses_with_gap_limit(gap_limit, true);
            self.register_addresses_with_gap_limit(gap_limit, false);
        }
    }

    // Derivation paths are composed of chains of addresses. Each chain is traversed until a gap of a certain number of addresses is
    // found that haven't been used in any transactions. This method returns an array of <gapLimit> unused addresses
    // following the last used address in the chain. The internal chain is used for change addresses and the external chain
    // for receive addresses.  These have a hardened purpose scheme depending on the derivation path

    pub fn register_addresses_with_gap_limit(&mut self, gap_limit: usize, internal: bool) -> Option<Vec<String>>{

        if !self.account.wallet.isTransient {
            assert!(self.base.addresses_loaded, "addresses must be loaded before calling this function");
        }
        let mut arr = if internal { self.internal_addresses.clone() } else { self.external_addresses.clone() };
        let mut i = arr.len();

        // keep only the trailing contiguous block of addresses with no transactions
        while i > 0 && !self.base.used_addresses.contains(&arr[i-1]) {
            i -= 1;
        }

        if i > 0 {
            arr.drain(0..i);
        }

        if arr.len() >= gapLimit {
            return Some(arr[0..gap_limit].to_owned());
        }

        if gapLimit > 1 { // get receive_address and change_address first to avoid blocking
            self.receive_address();
            self.change_address();
        }
        // @synchronized(self) {
        // It seems weird to repeat this, but it's correct because of the original call receive address and change address
        arr.clear();
        arr.extend(if internal { self.internal_addresses.clone() } else { self.external_addresses.clone() });
        i = arr.len();
        let n = i;

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
        let mut add_addresses = HashMap::new();

        while arr.len() < gap_limit { // generate new addresses up to gapLimit
            let pub_key = self.public_key_data_at_index(n, internal);
            if let Some(addr) = Key::address_with_public_key_data(pub_key, self.base.chain.chain_type.public_key_address()) {
                self.base.all_addresses.insert(addr.clone());
                if internal {
                    self.internal_addresses.push(addr.clone());
                } else {
                    self.external_addresses.push(addr.clone());
                }
                arr.push(addr.clone());
                add_addresses.insert(n, addr.clone());
            } else {
                println!("error generating keys");
                /*if (error) {
                    *error = [NSError errorWithDomain:@"DashSync"
                    code:500
                    userInfo:@{NSLocalizedDescriptionKey:
                        DSLocalizedString(@"Error generating public keys", nil)}];
                }*/
                return None;
            }
        }

        if !self.base.account.unwrap().wallet.unwrap().is_transient {
            //         [self.managedObjectContext performBlock:^{ // store new address in core data
            //             DSDerivationPathEntity *derivationPathEntity = [DSDerivationPathEntity derivationPathEntityMatchingDerivationPath:self inContext:self.managedObjectContext];
            //             for (NSNumber *number in addAddresses) {
            //                 NSString *address = [addAddresses objectForKey:number];
            //                 DSAddressEntity *e = [DSAddressEntity managedObjectInContext:self.managedObjectContext];
            //                 e.derivationPath = derivationPathEntity;
            //                 NSAssert([address isValidDashAddressOnChain:self.chain], @"the address is being saved to the wrong derivation path");
            //                 e.address = address;
            //                 e.index = [number intValue];
            //                 e.internal = internal;
            //                 e.standalone = NO;
            //             }
            //             [self.managedObjectContext ds_save];
            //         }];
        }
        Some(arr)
        // }
    }


    pub fn register_transaction_address(&mut self, address: String) -> bool {
        if self.base.contains_address(Some(address.clone())) {
            if !self.base.used_addresses.contains(&address) {
                self.base.used_addresses.insert(address.clone());
                if self.internal_addresses.contains(&address) {
                    self.register_addresses_with_gap_limit(GapLimit::Sequence.internal(), true);
                } else {
                    self.register_addresses_with_gap_limit(GapLimit::Sequence.external(), false);
                }
            }
            true
        }
        false
    }


    // returns the first unused external address
    pub fn receive_address(&mut self) -> Option<&String> {
        // TODO: limit to 10,000 total addresses and utxos for practical usability with bloom filters
        if let Some(addrs) = self.register_addresses_with_gap_limit(1, false) {
            if let Some(addr) = addrs.last() {
                return Some(addr);
            }
        }
        self.external_addresses.last()
    }

    pub fn receive_address_at_offset(&mut self, offset: usize) -> Option<&String> {
        // TODO: limit to 10,000 total addresses and utxos for practical usability with bloom filters
        if let Some(addrs) = self.register_addresses_with_gap_limit(offset + 1, false) {
            if let Some(addr) = addrs.last() {
                return Some(addr);
            }
        }
        self.external_addresses.last()
    }

    // returns the first unused internal address
    pub fn change_address(&mut self) -> Option<&String> {
        // TODO: limit to 10,000 total addresses and utxos for practical usability with bloom filters
        if let Some(addresses) = self.register_addresses_with_gap_limit(1, true) {
            return addresses.last();
        }
        None
    }

    // gets an address at an index one level down based on bip32
    pub fn address_at_index(&self, index: usize, internal: bool) -> Option<String> {
        let public_key = self.public_key_data_at_index(index, internal);
        if let Some(key) = ECDSAKey::key_with_public_key(public_key) {
            Key::address_with_public_key_data(
                key.base.public_key_data,
                self.base.chain.chain_type.public_key_address())
        } else {
            None
        }
    }

    pub fn addresses_for_export_with_internal_range(&self, internal_range: Range<usize>, external_range: Range<usize>) -> Vec<String> {
        let mut addresess: Vec<String> = Vec::new();
        internal_range.for_each(|i| {
            if let Some(address) = self.address_at_index(i, true) {
                addresess.push(address);
            }
        });
        external_range.for_each(|i| {
            if let Some(address) = self.address_at_index(i, false) {
                addresess.push(address);
            }
        });
        addresess
    }


    // all previously generated external addresses
    pub fn all_receive_addresses(&self) -> Vec<String> {
        self.external_addresses.clone()
    }

    // all previously generated internal addresses
    pub fn all_change_addresses(&self) -> Vec<String> {
        self.internal_addresses.clone()
    }

    // true if the address is controlled by the wallet
    pub fn contains_receive_address(&self, address: Option<String>) -> bool {
        address.is_some() && self.external_addresses.contains(&address.unwrap())
    }

    // true if the address is controlled by the wallet
    pub fn contains_change_address(&self, address: Option<String>) -> bool {
        address.is_some() && self.internal_addresses.contains(&address.unwrap())
    }

    pub fn used_receive_addresses(&self) -> Vec<String> {
        self.external_addresses.intersect(self.base.used_addresses.into_iter().collect())
    }

    pub fn used_change_addresses(&self) -> Vec<String> {
        self.internal_addresses.intersect(self.base.used_addresses.into_iter().collect())
    }

    pub fn private_key_string_at_index(&self, n: usize, internal: bool, seed: Option<Vec<u8>>) -> Option<&String> {
        self.serialized_private_keys(vec![n], internal, seed)?.last()
    }

    pub fn private_keys(&self, n: Vec<usize>, internal: bool, seed: Option<Vec<u8>>) -> Option<Vec<IndexPath<usize>>> {
        self.base.private_keys_at_index_paths(
            n
                .iter()
                .map(|index| IndexPath { indexes: vec![if internal {1} else {0}, index], length: 2 })
                .collect(),
            seed)
    }

    pub fn public_key_data_at_index(&self, n: usize, internal: bool) -> Option<Vec<u8>> {
        self.base.public_key_data_at_index_path(
            IndexPath {
                indexes: vec![if internal { 1 } else { 0 }, n],
                length: 2
            })
    }

    pub fn serialized_private_keys(&self, n: Vec<usize>, internal: bool, seed: Option<Vec<u8>>) -> Option<Vec<String>> {
        self.base.serialized_private_keys_at_index_paths(
            Some(n
                .iter()
                .map(|index| IndexPath {
                    indexes: vec![internal as usize, index],
                    length: 2
                })
                .collect()),
            seed)
    }

    pub fn index_path_for_known_address(&self, address: String) -> Option<IndexPath<usize>> {
        if let Some(index) = self.all_change_addresses().iter().position(|&a| a == address) {
            Some(IndexPath { indexes: vec![1, index], length: 2 })
        } else if let Some(index) = self.all_receive_addresses().iter().position(|&a| a == address) {
            Some(IndexPath { indexes: vec![0, index], length: 2 })
        } else {
            None
        }
    }
}

impl<P> DerivationPath<P> for FundsPath<P> {

}
