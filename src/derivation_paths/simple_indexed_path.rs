use std::ops::Range;
use dash_spv_primitives::crypto::index_path::IndexPath;
use dash_spv_primitives::crypto::UInt256;
use crate::chain::chain_parameters::ChainParameters;
use crate::{derivation_paths, keys};
use crate::chain::chain::Chain;
use crate::derivation_paths::{Path, Reference};
use crate::derivation_paths::path::DerivationPath;
use crate::keys::{IKey, Key};

pub struct SimpleIndexedPath<P: ChainParameters> {
    pub base: Path<P>,
    pub ordered_addresses: Vec<String>,
}

impl<P> SimpleIndexedPath<P> {

    pub fn init_with_indexes(
        indexes: Vec<UInt256>,
        hardened_indexes: Vec<bool>,
        r#type: derivation_paths::r#type::Type,
        signing_algorithm: keys::Type,
        reference: Reference,
        chain: Chain<P>
    ) -> Self<P> {
        Self {
            base: Path::init_with_indexes(indexes, hardened_indexes, r#type, signing_algorithm, reference, chain),
            ordered_addresses: vec![]
        }
    }

    // returns the index of an address in the derivation path as long as it is within the gap limit
    pub fn index_of_known_address(&self, address: String) -> Option<usize> {
        self.ordered_addresses.iter().position(|&p| p == address)
    }

    pub fn index_path_for_known_address(&self, address: String) -> Option<IndexPath<usize>> {
        if let Some(index) = self.index_of_known_address(address) {
            Some(IndexPath::index_path_with_index(index))
        } else {
            None
        }
    }

    // returns the index of the first unused Address;
    pub fn first_unused_index(&self) -> usize {
        let mut i = self.ordered_addresses.len();
        // keep only the trailing contiguous block of addresses that aren't used
        while i > 0 && self.base.used_addresses.contains(&self.ordered_addresses[i - 1]) {
            i -= 1;
        }
        i
    }

    // gets a public key at an index
    pub fn public_key_data_at_index(&self, index: usize) -> Option<Vec<u8>> {
        self.base.public_key_data_at_index_path(IndexPath::index_path_with_index(index))
    }

    // gets public keys to an index as NSData
    pub fn public_key_data_array_to_index(&self, index: usize) -> Vec<u8> {
        (0..index).into_iter().filter_map(|i| self.public_key_data_at_index(i)).collect()
    }

    // gets an addess at an index
    pub fn address_at_index(&self, index: usize) -> Option<String> {
        self.base.address_at_index_path(IndexPath::index_path_with_index(index))
    }

    // true if the address at the index was previously used as an input or output in any wallet transaction
    pub fn address_is_used_at_index(&self, index: usize) -> bool {
        self.base.address_is_used_at_index_path(IndexPath::index_path_with_index(index))
    }

    // gets addresses to an index, does not use cache and does not add to cache
    pub fn addresses_to_index(&mut self, index: usize) -> Vec<&String> {
        self.addresses_to_index_use_cache(index, false, false)
    }

    // gets addresses to an index, does not use cache and does not add to cache
    pub fn addresses_to_index_use_cache(&mut self, index: usize, use_cache: bool, add_to_cache: bool) -> Vec<&String> {
        let mut arr: Vec<&String> = Vec::new();
        (0..index).for_each(|i| {
            if use_cache && self.ordered_addresses.len() > i && self.ordered_addresses.get(i).is_some() {
                arr.push(self.ordered_addresses.get(i).unwrap());
            } else {
                let pub_key = self.public_key_data_at_index(i);
                if let Some(addr) = Key::address_with_public_key_data(pub_key, self.base.chain.chain_type.public_key_address()) {
                    arr.push(&addr);
                    if add_to_cache && self.ordered_addresses.len() == i {
                        self.ordered_addresses.push(addr);
                    }
                }
            }
        });
        arr
    }

    // gets a private key at an index
    pub fn private_key_at_index(&self, index: usize, seed: Option<Vec<u8>>) -> Option<dyn IKey<P>> {
        self.base.private_key_at_index_path(Some(IndexPath::index_path_with_index(index)), seed)
    }

    // get private keys for a range or to an index
    pub fn private_keys_to_index(&self, index: usize, seed: Option<Vec<u8>>) -> Vec<dyn IKey<P>> {
        self.private_keys_for_range(0..index, seed)
    }
    pub fn private_keys_for_range(&self, range: Range<usize>, seed: Option<Vec<u8>>) -> Vec<dyn IKey<P>> {
        range.into_iter().map(|i| self.private_key_at_index(i, seed)).collect()
    }

    // update addresses
    pub fn register_addresses_with_default_gap_limit_with_error(&mut self) -> Option<Vec<String>> {
        self.register_addresses_with_gap_limit(self.default_gap_limit())
    }

    pub fn register_addresses_with_gap_limit(&mut self, gap_limit: usize) -> Option<Vec<String>> {
        assert_ne!(self.base.r#type, derivation_paths::r#type::Type::MultipleUserAuthentication, "This should not be called for multiple user authentication. Use '- (NSArray *)register_addresses_with_gap_limit:(NSUInteger)gapLimit forIdentityIndex:(uint32_t)identityIndex error:(NSError**)error' instead.");
        let mut arr = self.ordered_addresses.clone();
        if !self.base.wallet.unwrap().is_transient {
            assert!(self.base.addresses_loaded, "addresses must be loaded before calling this function");
        }

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

        // @synchronized(self) {
        // It seems weird to repeat this, but it's correct because of the original call receive address and change address
        arr = self.ordered_addresses.clone();
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

        while arr.len() < gap_limit { // generate new addresses up to gapLimit
            let pub_key = self.public_key_data_at_index(n);
            if let Some(addr) = Key::address_with_public_key_data(pub_key, self.base.chain.chain_type.public_key_address()) {
                if !self.wallet.is_transient {

                    /*[self.managedObjectContext performBlock:^{ // store new address in core data
                        DSDerivationPathEntity *derivationPathEntity = [DSDerivationPathEntity derivationPathEntityMatchingDerivationPath:self inContext:self.managedObjectContext];
                        DSAddressEntity *e = [DSAddressEntity managedObjectInContext:self.managedObjectContext];
                        e.derivationPath = derivationPathEntity;
                        NSAssert([addr isValidDashAddressOnChain:self.chain], @"the address is being saved to the wrong derivation path");
                        e.address = addr;
                        e.index = n;
                        e.standalone = NO;
                    }];*/
                }

                self.base.all_addresses.insert(addr.clone());
                arr.push(addr.clone());
                self.ordered_addresses.push(addr.clone());
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
        return rArray;
        // }
    }
    // inherited
    pub fn register_transaction_address(&mut self, address: String) -> bool {
        let has_addr = self.base.contains_address(Some(address));
        if has_addr {
            if !self.base.used_addresses.contains(&address) {
                self.base.used_addresses.insert(address.clone());
                self.register_addresses_with_default_gap_limit_with_error();
            }
        }
        has_addr
    }

    pub fn reload_addresses(&mut self) {
        self.base.all_addresses.clear();
        self.ordered_addresses.clear();
        self.base.used_addresses.clear();
        self.base.addresses_loaded = false;
        self.base.load_addresses();
    }

    fn default_gap_limit(&self) -> usize {
        10
    }
}

impl<P> DerivationPath<P> for SimpleIndexedPath<P> {

}
