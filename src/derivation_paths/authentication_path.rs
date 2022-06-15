use std::collections::HashMap;
use dash_spv_primitives::crypto::data_ops::uint256_from_long;
use dash_spv_primitives::crypto::index_path::IndexPath;
use dash_spv_primitives::crypto::{UInt160, UInt256};
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::{derivation_paths, keys};
use crate::derivation_paths::{Path, Reference};
use crate::derivation_paths::path::{BIP32_HARD, FEATURE_PURPOSE, FEATURE_PURPOSE_IDENTITIES, FEATURE_PURPOSE_IDENTITIES_SUBFEATURE_AUTHENTICATION};
use crate::derivation_paths::simple_indexed_path::SimpleIndexedPath;
use crate::keys::{IKey, Key, Type};

pub struct AuthenticationPath<P: ChainParameters> {
    pub base: SimpleIndexedPath<P>,
    pub uses_hardened_keys: bool,
    should_store_extended_private_key: bool,
    addresses_by_identity: HashMap<u32, Vec<String>>,
}

impl<P> AuthenticationPath<P> {

    pub fn authentication_path_with_indexes(
        indexes: Vec<UInt256>,
        hardened_indexes: Vec<bool>,
        r#type: derivation_paths::r#type::Type,
        signing_algorithm: Type,
        reference: Reference,
        uses_hardened_keys: bool,
        should_store_extended_private_key: bool,
        chain: Chain<P>) -> Self<P> {
        Self {
            base: SimpleIndexedPath::init_with_indexes(indexes, hardened_indexes, r#type, signing_algorithm, reference, chain),
            uses_hardened_keys,
            should_store_extended_private_key,
            addresses_by_identity: HashMap::new()
        }
    }


    pub fn derivation_path_with_indexes(indexes: Vec<UInt256>, hardened_indexes: Vec<bool>, r#type: derivation_paths::r#type::Type, signing_algorithm: Type, reference: Reference, chain: Chain<P>) -> Self<P> {
        Self::authentication_path_with_indexes(
            indexes,
            hardened_indexes,
            r#type,
            signing_algorithm,
            reference,
            true,
            false,
            chain)
    }

    pub fn provider_voting_keys_derivation_path_for_chain(chain: Chain<P>) -> Self<P> {
        Self::authentication_path_with_indexes(
            vec![
                uint256_from_long(FEATURE_PURPOSE),
                uint256_from_long(coin_type),
                uint256_from_long(3),
                uint256_from_long(1)
            ],
            vec![true, true, true, true],
            derivation_paths::r#type::Type::SingleUserAuthentication,
            Type::ECDSA,
            Reference::ProviderVotingKeys,
            false,
            false,
            chain)
    }

    pub fn provider_owner_keys_derivation_path_for_chain(chain: Chain<P>) -> Self<P> {
        Self::authentication_path_with_indexes(
            vec![
                uint256_from_long(FEATURE_PURPOSE),
                uint256_from_long(coin_type),
                uint256_from_long(3),
                uint256_from_long(2)
            ],
            vec![true, true, true, true],
            derivation_paths::r#type::Type::SingleUserAuthentication,
            Type::ECDSA,
            Reference::ProviderOwnerKeys,
            false,
            false,
            chain)
    }

    pub fn provider_operator_keys_derivation_path_for_chain(chain: Chain<P>) -> Self<P> {
        Self::authentication_path_with_indexes(
            vec![
                uint256_from_long(FEATURE_PURPOSE),
                uint256_from_long(coin_type),
                uint256_from_long(3),
                uint256_from_long(3)
            ],
            vec![true, true, true, true],
            derivation_paths::r#type::Type::SingleUserAuthentication,
            Type::BLS,
            Reference::ProviderOperatorKeys,
            false,
            false,
            chain)
    }

    pub fn blockchain_identity_ecdsa_keys_derivation_path_for_chain(chain: Chain<P>) -> Self<P> {
        Self::authentication_path_with_indexes(
            vec![
                uint256_from_long(FEATURE_PURPOSE),
                uint256_from_long(coin_type),
                uint256_from_long(FEATURE_PURPOSE_IDENTITIES),
                uint256_from_long(FEATURE_PURPOSE_IDENTITIES_SUBFEATURE_AUTHENTICATION),
                uint256_from_long(0)
            ],
            hardened_indexes: vec![true, true, true, true, true],
            derivation_paths::r#type::Type::MultipleUserAuthentication,
            Type::ECDSA,
            Reference::BlockchainIdentities,
            true,
            true,
            chain)
    }



    pub fn blockchain_identity_bls_keys_derivation_path_for_chain(chain: Chain<P>) -> Self<P> {
        Self::authentication_path_with_indexes(
            vec![
                uint256_from_long(FEATURE_PURPOSE),
                uint256_from_long(chain.chain_type.coin_type()),
                uint256_from_long(FEATURE_PURPOSE_IDENTITIES),
                uint256_from_long(FEATURE_PURPOSE_IDENTITIES_SUBFEATURE_AUTHENTICATION),
                uint256_from_long(1)
            ],
            hardened_indexes: vec![true, true, true, true, true],
            derivation_paths::r#type::Type::MultipleUserAuthentication,
            Type::BLS,
            Reference::BlockchainIdentities,
            true,
            true,
            chain)
    }



    pub fn load_addresses(&mut self) {
        if self.base.base.addresses_loaded {
            return;
        }
        // [self.managedObjectContext performBlockAndWait:^{
        //     DSDerivationPathEntity *derivationPathEntity = [DSDerivationPathEntity derivationPathEntityMatchingDerivationPath:self inContext:self.managedObjectContext];
        //     self.syncBlockHeight = derivationPathEntity.syncBlockHeight;
        //     NSArray<DSAddressEntity *> *addresses = [derivationPathEntity.addresses sortedArrayUsingDescriptors:@[[NSSortDescriptor sortDescriptorWithKey:@"index" ascending:YES]]];
        //     for (DSAddressEntity *e in addresses) {
        //         @autoreleasepool {
        //             while (e.index >= self.mOrderedAddresses.count) [self.mOrderedAddresses addObject:[NSNull null]];
        //             if (![e.address isValidDashAddressOnChain:self.wallet.chain]) {
        //             #if DEBUG
        //             DSLogPrivate(@"address %@ loaded but was not valid on chain %@", e.address, self.wallet.chain.name);
        //             #else
        //             DSLog(@"address %@ loaded but was not valid on chain %@", @"<REDACTED>", self.wallet.chain.name);
        //             #endif
        //             continue;
        //             }
        //             self.mOrderedAddresses[e.index] = e.address;
        //             [self.mAllAddresses addObject:e.address];
        //             if ([e.usedInInputs count] || [e.usedInOutputs count] || [e.usedInSpecialTransactions count] || [e.usedInSimplifiedMasternodeEntries count]) {
        //             [self.mUsedAddresses addObject:e.address];
        //             }
        //         }
        //     }
        // }];

        self.base.base.addresses_loaded = true;

        if self.base.base.r#type == derivation_paths::r#type::Type::SingleUserAuthentication {
            self.base.register_addresses_with_gap_limit(10);
        } else {
            self.register_addresses_with_gap_limit(10, 0);
        }

    }

    pub fn register_addresses_with_gap_limit(&mut self, gap_limit: usize, identity_index: u32) -> Option<Vec<String>> {
        if !self.base.base.account.unwrap().wallet.unwrap().is_transient {
            assert!(self.base.base.addresses_loaded, "addresses must be loaded before calling this function");
        }
        assert_ne!(self.base.base.r#type, derivation_paths::r#type::Type::SingleUserAuthentication, "This should not be called for single user authentication. Use '- (NSArray *)registerAddressesWithGapLimit:(NSUInteger)gapLimit error:(NSError**)error' instead.");

        if self.uses_hardened_keys && !self.has_extended_private_key() {
            return Some(vec![]);
        }

        if self.addresses_by_identity.get(&identity_index).is_none() {
            self.addresses_by_identity.insert(identity_index, vec![]);
        }

        let mut a = self.addresses_by_identity.get(&identity_index).unwrap().clone();
        let mut i = a.len();

        // keep only the trailing contiguous block of addresses with no transactions
        while i > 0 && !self.base.base.used_addresses.contains(&a[i-1]) {
            i -= 1;
        }

        if i > 0 {
            a.drain(0..i);
        }

        if a.len() >= gap_limit {
            return Some(a[0..gap_limit].to_owned());
        }

        // TODO: make sure we need to use mutex or some kind of another lock here
        //@synchronized(self) {

        // It seems weird to repeat this, but it's correct because of the original call receive address and change address
        a.clear();
        a.extend(self.addresses_by_identity.get(&identity_index).unwrap());
        i = a.len();
        let mut n = i;

        // keep only the trailing contiguous block of addresses with no transactions
        while i > 0 && !self.base.base.used_addresses.contains(&a[i-1]) {
            i -= 1;
        }

        if i > 0 {
            a.drain(0..i);
        }
        if a.len() >= gap_limit {
            return Some(a[0..gap_limit].to_owned());
        }

        while a.len() < gap_limit { // generate new addresses up to gapLimit
            let hardened_indexes: Vec<usize> = vec![identity_index as usize | BIP32_HARD, n | BIP32_HARD];
            let soft_indexes: Vec<usize> = vec![identity_index as usize, n];
            let indexes = if self.uses_hardened_keys { hardened_indexes } else { soft_indexes };
            let pub_key = self.public_key_data_at_index_path(IndexPath::index_path_with_indexes(indexes));
            if let Some(addr) = Key::address_with_public_key_data(pub_key, self.base.base.chain.chain_type.public_key_address()) {
                if !self.base.base.account.unwrap().wallet.unwrap().is_transient {
                    // [self.managedObjectContext performBlock:^{ // store new address in core data
                    //     DSDerivationPathEntity *derivationPathEntity = [DSDerivationPathEntity derivationPathEntityMatchingDerivationPath:self inContext:self.managedObjectContext];
                    //     DSAddressEntity *e = [DSAddressEntity managedObjectInContext:self.managedObjectContext];
                    //     e.derivationPath = derivationPathEntity;
                    //     NSAssert([addr isValidDashAddressOnChain:self.chain], @"the address is being saved to the wrong derivation path");
                    //     e.address = addr;
                    //     e.index = n;
                    //     e.identityIndex = identityIndex;
                    //     e.standalone = NO;
                    // }];
                }
                self.base.base.all_addresses.insert(addr.clone());
                self.addresses_by_identity.entry(identity_index).or_insert(vec![]).push(addr.clone());
                a.push(addr.clone());
                n += 1;
            } else {
                println!("error generating keys");
                // if (error) {
                //     *error = [NSError errorWithDomain:@"DashSync"
                //     code:500
                //     userInfo:@{NSLocalizedDescriptionKey:
                //         DSLocalizedString(@"Error generating public keys", nil)}];
                // }
                // return nil;
                return None;
            }
        }
        Some(a)
        //}

    }


    pub fn first_unused_public_key(&self) -> Option<Vec<u8>> {
        self.base.public_key_data_at_index(self.base.first_unused_index())
    }

    pub fn first_unused_private_key_from_seed(&self, seed: Option<Vec<u8>>) -> Option<Vec<u8>> {
        self.base.base.private_key_at_index_path(Some(IndexPath::index_path_with_index(self.base.first_unused_index())), seed)
    }

    pub fn private_key_for_address(&self, address: String, seed: Option<Vec<u8>>) -> Option<dyn IKey<P>> {
        if let Some(index) = self.base.index_of_known_address(address) {
            self.base.base.private_key_at_index_path(Some(IndexPath::index_path_with_index(index)), seed)
        } else {
            None
        }
    }

    pub fn public_key_data_for_address(&self, address: String) -> Option<Vec<u8>> {
        if let Some(index) = self.base.index_of_known_address(address) {
            self.base.public_key_data_at_index(index)
        } else {
            None
        }
    }

    pub fn private_key_for_hash_160(&self, hash: UInt160, seed: Option<Vec<u8>>) -> Option<dyn IKey<P>> {
        if let Some(address) = Key::address_from_hash_160_data(Some(hash.0.to_vec()), self.base.base.chain.chain_type.public_key_address()) {
            self.private_key_for_address(address, seed)
        } else {
            None
        }
    }

    pub fn public_key_data_for_hash_160(&self, hash: UInt160) -> Option<Vec<u8>> {
        if let Some(address) = Key::address_from_hash_160_data(Some(hash.0.to_vec()), self.base.base.chain.chain_type.public_key_address()) {
            self.public_key_data_for_address(address)
        } else {
            None
        }
    }

    pub fn generate_extended_public_key_from_seed(&mut self, seed: Option<Vec<u8>>, wallet_unique_id: Option<String>) -> Option<dyn IKey<P>> {
        self.base.base.generate_extended_public_key_from_seed_and_store_private_key(seed, wallet_unique_id, self.should_store_extended_private_key)
    }

    fn has_extended_private_key(&self) -> bool {
        false
        // NSError *error = nil;
        // return hasKeychainData([self walletBasedExtendedPrivateKeyLocationString], &error);
    }

    fn extended_privaete_key_data(&self) -> Option<Vec<u8>> {
        None
        // NSError *error = nil;
        // NSData *data = getKeychainData([self walletBasedExtendedPrivateKeyLocationString], &error);
        // return data;
    }

    fn private_key_at_index_path(&self, index_path: IndexPath<usize>) -> Option<dyn IKey<P>> {
        if let Some(extended_private_key) = Key::key_with_extended_private_key_data(self.extended_privaete_key_data(), self.base.base.signing_algorithm) {
            extended_private_key.private_derive_to_path(index_path)
        } else {
            None
        }
    }

    // inherited
    pub fn public_key_data_at_index_path(&self, index_path: IndexPath<usize>) -> Option<Vec<u8>> {
        let mut has_hardened_derivation = false;
        for i in 0..index_path.length {
            let derivation = index_path.index_at_position(i);
            has_hardened_derivation |= derivation & BIP32_HARD > 0;
            if has_hardened_derivation {
                break;
            }
        }
        if has_hardened_derivation {
            if self.has_extended_private_key() {
                if let Some(key) = self.private_key_at_index_path(index_path) {
                    return Some(key.get_public_key_data());
                }
            }
            None
        } else {
            self.base.base.public_key_data_at_index_path(index_path)
        }
    }

    fn default_gap_limit(&self) -> usize {
        10
    }
}

impl<P> DerivationPath<P> for AuthenticationPath<P> {

}
