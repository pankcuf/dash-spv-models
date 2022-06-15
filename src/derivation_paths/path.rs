use std::borrow::BorrowMut;
use std::collections::HashSet;
use byte::BytesExt;
use dash_spv_primitives::crypto::byte_util::ECPoint;
use dash_spv_primitives::crypto::data_ops::{bip32, deserialize, uint256_from_long};
use dash_spv_primitives::crypto::index_path::IndexPath;
use dash_spv_primitives::crypto::UInt256;
use crate::chain::chain::Chain;
use crate::common::ChainType;
use crate::common::ChainType::MainNet;
use crate::derivation_paths::reference::Reference;
use crate::{derivation_paths, keys};
use crate::chain::chain_parameters::ChainParameters;
use crate::derivation_paths::r#type;
use crate::keys::{BLSKey, ECDSAKey, IKey, Key, Type};
use crate::wallet::account::Account;
use crate::wallet::wallet::Wallet;

pub const BIP32_HARD: usize = 0x80000000;
pub const BIP32_HARD_LE: usize = 0x00000080;

pub const FEATURE_PURPOSE: u64 = 9;
pub const FEATURE_PURPOSE_IDENTITIES: u64 = 5;
pub const FEATURE_PURPOSE_IDENTITIES_SUBFEATURE_AUTHENTICATION: u64 = 0;
pub const FEATURE_PURPOSE_IDENTITIES_SUBFEATURE_REGISTRATION: u64 = 1;
pub const FEATURE_PURPOSE_IDENTITIES_SUBFEATURE_TOPUP: u64 = 2;
pub const FEATURE_PURPOSE_IDENTITIES_SUBFEATURE_INVITATIONS: u64 = 3;
pub const FEATURE_PURPOSE_DASHPAY: u64 = 15;

const DERIVATION_PATH_EXTENDED_PUBLIC_KEY_WALLET_BASED_LOCATION: &str = "DP_EPK_WBL";
const DERIVATION_PATH_EXTENDED_PUBLIC_KEY_STANDALONE_BASED_LOCATION: &str = "DP_EPK_SBL";
const DERIVATION_PATH_EXTENDED_SECRET_KEY_WALLET_BASED_LOCATION: &str = "DP_ESK_WBL";
const DERIVATION_PATH_STANDALONE_INFO_DICTIONARY_LOCATION: &str = "DP_SIDL";
const DERIVATION_PATH_STANDALONE_INFO_TERMINAL_INDEX: &str = "DP_SI_T_INDEX";
const DERIVATION_PATH_STANDALONE_INFO_TERMINAL_HARDENED: &str = "DP_SI_T_HARDENED";
const DERIVATION_PATH_STANDALONE_INFO_DEPTH: &str = "DP_SI_DEPTH";

// pub type TransactionValidityCompletionBlock = Fn(bool, bool);


// #[derive(Debug, Eq, PartialEq)]
// pub enum DerivationPathType {
//     Base,
//     SimpleIndexed,
//     Authentication,
//     CreditFunding,
//     Funds,
//     IncomingFunds,
//     MasternodeHolding,
// }

pub trait DerivationPath<P: ChainParameters> {
    fn get_base(&self) -> dyn DerivationPath<P>;
    fn get_type(&self) -> DerivationPathType;
    fn get_reference(&self) -> Reference;
}

pub struct Path<P: ChainParameters> {

    pub indexes: Vec<UInt256>,
    pub hardened_indexes: Vec<bool>,

    /// master public key used to generate wallet addresses
    pub(crate) extended_public_key: Option<dyn IKey<P>>,

    /// is this an open account
    pub r#type: r#type::Type,

    pub signing_algorithm: Type,

    /// account for the derivation path
    pub chain: Chain<P>,

    /// account for the derivation path
    pub account: Option<Account<P>>, // weak
    pub wallet: Option<Wallet<P>>, // weak

    /// this returns the derivation path's visual representation (e.g. m/44'/5'/0')
    pub string_representation: Option<String>,

    /// extended Public Key Identifier, which is just the short hex string of the extended public key
    pub standalone_extended_public_key_unique_id: Option<String>,

    /// the wallet_based_extended_public_key_location_string is the key used to store the public key in the key chain
    pub wallet_based_extended_public_key_location_string: Option<String>,

    /// the wallet_based_extended_public_key_location_string is the key used to store the private key in the key chain, this is only available on authentication derivation paths
    pub wallet_based_extended_private_key_location_string: Option<String>,

    /// current derivation path balance excluding transactions known to be invalid
    pub balance: u64,

    /// currently the derivationPath is synced to this block height
    pub sync_block_height: u32,

    /// all previously generated addresses
    pub all_addresses: HashSet<String>,

    /// all previously used addresses
    pub used_addresses: HashSet<String>,


    /// the reference of type of derivation path
    pub reference: Reference,

    /// there might be times where the derivationPath is actually unknown, for example when importing from an extended public key
    pub derivation_path_is_known: bool,


    ////
    pub addresses_loaded: bool,
    pub depth: u8,
}

impl<P> Path<P> {
    /// UInt256IndexPath

    pub fn index_at_position(&self, position: usize) -> UInt256 {
        if position >= self.indexes.len() {
            UInt256::MAX
        } else {
            self.indexes[position]
        }
    }

    /// Purpose
    pub fn is_bip32_only(&self) -> bool {
        self.indexes.len() == 1
    }

    pub fn is_bip43_based(&self) -> bool {
        !self.is_bip32_only()
    }

    /// purpose of the derivation path if BIP 43 based
    pub fn purpose(&self) -> u64 {
        if self.is_bip43_based() {
            self.index_at_position(0).0.read_with::<u64>(&mut 0, byte::LE).unwrap()
        } else {
            0
        }
    }

    pub fn is_hardened_at_position(&self, position: usize) -> bool {
        if position >= self.indexes.len() {
            false
        } else {
            self.hardened_indexes[position]
        }
    }

    pub fn has_extended_public_key(&self) -> bool {
        if self.extended_public_key.is_some() {
            return true;
        }
        false
        // hasKeychainData(if self.wallet.is_some() &&
        //     (self.indexes.len() > 0 || self.reference == Reference::Root) {
        //     self.wallet_based_extended_public_key_location_string
        // } else {
        //     self.standalone_extended_public_key_unique_id
        // }, nil)
    }

    pub fn private_key_at_index_path(&self, index_path: Option<IndexPath<usize>>, seed: Option<Vec<u8>>) -> Option<dyn IKey<P>> {
        assert!(index_path);
        assert!(seed);
        match index_path {
            Some(index_path) => {
                if seed.is_none() {
                    return None;
                }
                //if (![self length]) return nil; //there needs to be at least 1 length
                assert!(top_key, "Top key should exist");
                return if let Some(top_key) = Key::key_with_seed_data(seed.clone(), self.signing_algorithm.clone()) {
                    if let Some(derivation_path_extended_key) = top_key.private_derive_to_256_bit_derivation_path(*self) {
                        derivation_path_extended_key.private_derive_to_path(index_path)
                    } else {
                        assert!(false, "Top key should exist");
                        None
                    }
                } else {
                    assert!(false, "Top key should exist");
                    None
                }
            },
            _ => None
        }
    }

    pub fn get_string_representation_of_index(index: UInt256, hardened: bool) {
        if uin2
    }

    // + (NSString *)stringRepresentationOfIndex:(UInt256)index hardened:(BOOL)hardened inContext:(NSManagedObjectContext *)context {
    // if (uint256_is_31_bits(index)) {
    // return [NSString stringWithFormat:@"/%lu%@", (unsigned long)index.u64[0], hardened ? @"'" : @""];
    // } else if (context) {
    // __block NSString *rString = nil;
    // [context performBlockAndWait:^{
    // DSDashpayUserEntity *dashpayUserEntity = [DSDashpayUserEntity anyObjectInContext:context matching:@"associatedBlockchainIdentity.uniqueID == %@", uint256_data(index)];
    // if (dashpayUserEntity) {
    // DSBlockchainIdentityUsernameEntity *usernameEntity = [dashpayUserEntity.associatedBlockchainIdentity.usernames anyObject];
    // rString = [NSString stringWithFormat:@"/%@%@", usernameEntity.stringValue, hardened ? @"'" : @""];
    // } else {
    // rString = [NSString stringWithFormat:@"/0x%@%@", uint256_hex(index), hardened ? @"'" : @""];
    // }
    // }];
    // return rString;
    // } else {
    // return [NSString stringWithFormat:@"/0x%@%@", uint256_hex(index), hardened ? @"'" : @""];
    // }
    // }


    pub fn get_string_representation(&self) -> String {
        match self.string_representation {
            Some(representation) => representation,
            None => {
                let mut mutable_string = String::new();
                if self.indexes.len() > 0 {
                    self.indexes.iter().for_each(|i| {
                        let path = Self::get_string_representation_of_index
                        mutable_string.append()
                    })
                }

            }
        }

        if (_stringRepresentation) return _stringRepresentation;
        NSMutableString *mutableString = [NSMutableString stringWithFormat:@"m"];
        if (self.length) {
            for (NSInteger i = 0; i < self.length; i++) {
                [mutableString appendString:[DSDerivationPath stringRepresentationOfIndex:[self indexAtPosition:i] hardened:[self isHardenedAtPosition:i] inContext:self.managedObjectContext]];
            }
        } else if ([self.depth integerValue]) {
        for (NSInteger i = 0; i < [self.depth integerValue] - 1; i++) {
        [mutableString appendFormat:@"/?'"];
        }
        UInt256 terminalIndex = [self terminalIndex];
        BOOL terminalHardened = [self terminalHardened];
        [mutableString appendString:[DSDerivationPath stringRepresentationOfIndex:terminalIndex hardened:terminalHardened inContext:self.managedObjectContext]];
        } else {
        if ([self isKindOfClass:[DSIncomingFundsDerivationPath class]]) {
        mutableString = [NSMutableString stringWithFormat:@"inc"];
        DSIncomingFundsDerivationPath *incomingFundsDerivationPath = (DSIncomingFundsDerivationPath *)self;
        [self.managedObjectContext performBlockAndWait:^{
        DSDashpayUserEntity *sourceDashpayUserEntity = [DSDashpayUserEntity anyObjectInContext:self.managedObjectContext matching:@"associatedBlockchainIdentity.uniqueID == %@", uint256_data(incomingFundsDerivationPath.contactSourceBlockchainIdentityUniqueId)];
        if (sourceDashpayUserEntity) {
        DSBlockchainIdentityUsernameEntity *usernameEntity = [sourceDashpayUserEntity.associatedBlockchainIdentity.usernames anyObject];
        [mutableString appendFormat:@"/%@", usernameEntity.stringValue];
        } else {
        [mutableString appendFormat:@"/0x%@", uint256_hex(incomingFundsDerivationPath.contactSourceBlockchainIdentityUniqueId)];
        }
        }];
        DSBlockchainIdentity *blockchainIdentity = [self.wallet blockchainIdentityForUniqueId:incomingFundsDerivationPath.contactDestinationBlockchainIdentityUniqueId];
        [mutableString appendFormat:@"/%@", blockchainIdentity.currentDashpayUsername];
        }
        }
        _stringRepresentation = [mutableString copy];
        return _stringRepresentation;

    }

    /// extended Public Key
    pub fn extended_public_key_data(&self) -> Option<Vec<u8>> {
        self.extended_public_key?.get_base().extended_public_key_data
    }

    pub fn standalone_save_extended_public_key_to_key_chain(&self) {

    }

    // true if the address is controlled by the wallet
    pub fn contains_address(&self, address: Option<String>) -> bool {
        address.is_some() && self.all_addresses.contains(address.unwrap().as_str())
    }


    pub fn load_addresses(&self) {

    }
    pub fn reload_addresses(&self) {

    }

    pub fn private_keys_at_index_paths<T>(&self, index_paths: Option<Vec<IndexPath<T>>>, seed: Option<Vec<u8>>) -> Option<Vec<dyn IKey<P>>> {
        match index_paths {
            None => None,
            Some(index_paths) => {
                if seed.is_none() {
                    return None;
                }
                let private_keys: Vec<dyn IKey<P>> = Vec::with_capacity(index_paths.len());
                if index_paths.len() == 0 {
                    return Some(private_keys);
                }
                if let Some(top_key)= keys::Key::key_with_seed_data(seed, self.signing_algorithm.clone()) {
                    let derivation_path_extended_key = top_key.private_derive_to_256_bit_derivation_path(self)

                    #[cfg(debug_assertions)]
                    {
                        if let Some(public_key) = self.extended_public_key?.get_base().extended_public_key_data {
                            let public_key = self.extended_public_key
                        }
                    }
                }
            }
        }

        if index_paths.is_none() || seed.is_none() {
            return None;
        }
        if index_paths.
    }
    - (NSArray *)privateKeysAtIndexPaths:(NSArray *)indexPaths fromSeed:(NSData *)seed {
    if (!seed || !indexPaths) return nil;
    if (indexPaths.count == 0) return @[];
    NSMutableArray *privateKeys = [NSMutableArray arrayWithCapacity:indexPaths.count];
    DSKey *topKey = [DSKey keyWithSeedData:seed forKeyType:self.signingAlgorithm];
    DSKey *derivationPathExtendedKey = [topKey privateDeriveTo256BitDerivationPath:self];

    #if DEBUG
    if (_extendedPublicKey) {
    NSData *publicKey = _extendedPublicKey.extendedPublicKeyData;
    NSAssert([publicKey isEqualToData:derivationPathExtendedKey.extendedPublicKeyData], @"The derivation doesn't match the public key");
    }
    #endif

    for (NSIndexPath *indexPath in indexPaths) {
    DSKey *privateKey = [derivationPathExtendedKey privateDeriveToPath:indexPath];
    [privateKeys addObject:privateKey];
    }

    return privateKeys;
    }

    pub fn public_key_data_at_index_path(&self, index_path: IndexPath<usize>) -> Option<Vec<u8>> {
        self.signing_algorithm.public_key_data_at_index_path(self.extended_public_key_data(), index_path)
    }


    // gets an address at an index path
    pub fn address_at_index_path(&self, index_path: IndexPath<usize>) -> Option<String> {
        let pub_key = self.public_key_data_at_index_path(index_path);
        Key::address_with_public_key_data(pub_key, self.chain.chain_type.public_key_address())
    }

    // true if the address was previously used as an input or output in any wallet transaction
    pub fn address_is_used(&self, address: Option<String>) -> bool {
        address.is_some() && self.used_addresses.contains(&address.unwrap())
    }


    pub fn address_is_used_at_index_path(&self, index_path: IndexPath<usize>) -> bool {
        self.address_is_used(self.address_at_index_path(index_path))
    }

    pub fn serialized_private_keys_at_index_paths(&self, index_paths: Option<Vec<IndexPath<u32>>>, seed: Option<Vec<u8>>) -> Option<Vec<dyn IKey<P>>> {
        match (index_paths, seed) {
            (Some(index_paths), Some(seed)) => {
                let mut serialized_private_keys: Vec<dyn IKey<P>> = Vec::with_capacity(index_paths.len());
                if index_paths.is_empty() {
                    return Some(serialized_private_keys);
                }
                if let Some(top_key)= keys::Key::key_with_seed_data(Some(seed), self.signing_algorithm.clone()) {
                    if let Some(derivation_path_extended_key) = top_key.private_derive_to_256_bit_derivation_path(&self) {
                        index_paths.iter().map(|&index_path| {
                            if let Some(private_key) = derivation_path_extended_key.private_derive_to_path(index_path) {
                                let serialized_private_key = private_key.serialized_private_key_for_chain(self.chain);
                                assert!(serialized_private_key, "The serialized private key should exist");
                                serialized_private_keys.push(serialized_private_key.unwrap());
                            }
                        })
                    }
                }
                serialized_private_keys
            },
            _ => None

        }
    }

    pub fn index_path_for_known_address(&self, address: String) -> Option<IndexPath<usize>> {
        assert!(false, "This must be implemented in concrete derivation path {}", address);
        None
    }


    pub fn wallet_based_extended_public_key_location_string_for_unique_id(unique_id: &str) -> String {
        format!("{}_{}", DERIVATION_PATH_EXTENDED_PUBLIC_KEY_WALLET_BASED_LOCATION, unique_id)
    }

    pub fn wallet_based_extended_private_key_location_string_for_unique_id(unique_id: &str) -> String {
        format!("{}_{}", DERIVATION_PATH_EXTENDED_SECRET_KEY_WALLET_BASED_LOCATION, unique_id)
    }


    pub fn wallet_based_extended_public_key_location_string_for_wallet_unique_id(&self, unique_id: &str) -> String {
        let mut s = String::new();
        for i in 0..self.indexes.len() {
            let mut index = self.index_at_position(i).0.read_with::<usize>(&mut 0, byte::LE).unwrap();
            if self.is_hardened_at_position(i) {
                index |= bip32::HARD;
            }
            s = s.append(format!("_{}", index));
        }
        let path = Path::wallet_based_extended_public_key_location_string_for_unique_id(unique_id);
        let algo = if self.signing_algorithm == Type::BLS { "_BLS_" } else {""};
        format!("{}{}{}", path, algo, s)
    }

    pub fn master_blockchain_identity_contacts_derivation_path_for_account_number(account_number: u32, chain: Chain<P>) -> Self<P> {
        /// TODO: full uint256 derivation
        Self::derivation_path_with_indexes(
            vec![
                uint256_from_long(FEATURE_PURPOSE),
                uint256_from_long(chain.r#type.coin_type()),
                uint256_from_long(FEATURE_PURPOSE_DASHPAY),
                uint256_from_long(account_number)
            ],
            vec![true, true, true, true],
            derivation_paths::r#type::PartialPath,
            Type::ECDSA,
            Reference::ContactBasedFundsRoot,
            chain)
    }

    pub fn derivation_path_with_indexes(
        indexes: Vec<UInt256>,
        hardened_indexes: Vec<bool>,
        r#type: r#type::Type,
        signing_algorithm: Type,
        reference: Reference,
        chain: Chain<P>
    ) -> Self<P> {
        init_with_indexes(indexes, hardened_indexes, r#type, signing_algorithm, reference, chain)
    }

    pub fn derivation_path_with_serialized_extended_private_key(
        serialized_extended_private_key: &str,
        funds_type: derivation_paths::Type,
        chain: Chain<P>,
    ) -> Self<P> {
        let extended_private_key = deserialized_extended_private_key(
            serialized_extended_private_key,
            chain.clone()
        );
        let mut derivation_path = Self::init_with_indexes(
            Vec::new(),
            Vec::new(),
            funds_type,
            keys::Type::ECDSA,
            Reference::Unknown,
            chain);
        derivation_path.extended_public_key = Some(ECDSAKey::key_with_secret(extended_private_key, true));
        derivation_path.stan.standaloneSaveExtendedPublicKeyToKeyChain();
        derivation_path
    }

    pub fn derivation_path_with_serialized_extended_public_key(
        serialized_extended_public_key: &str,
        chain: Chain<P>,
    ) -> Self<P> {
        let mut depth: u8 = 0;
        let mut terminal_hardened = false;
        let mut terminal_index = UInt256::MIN;
        let extended_public_key_data = Self::deserialized_extended_public_key(
            serialized_extended_public_key,
            chain.clone(),
            depth.borrow_mut(),
            terminal_hardened.borrow_mut(),
            terminal_index.borrow_mut(),
        );
        let mut path = Self::init_with_indexes(
            vec![terminal_index],
            vec![terminal_hardened],
            derivation_paths::Type::ViewOnlyFunds,
            keys::Type::ECDSA,
            Reference::Unknown,
            chain);
        path.extended_public_key = Some(ECDSAKey::key_with_extended_public_key_data(extended_public_key_data));
        path.depth = depth;
        path.standalone_save_extended_public_key_to_key_chain();
        path.load_addresses();
        path
    }

    pub fn init_with_extended_public_key_identifier(identifier: &str, chain: Chain<P>) -> Self<P> {

        NSError *error = nil;
        NSDictionary *infoDictionary = getKeychainDict([DSDerivationPath standaloneInfoDictionaryLocationStringForUniqueID:extendedPublicKeyIdentifier], @[[NSString class], [NSNumber class]], &error);
        if (error) return nil;

        UInt256 terminalIndex = [((NSData *)infoDictionary[DERIVATION_PATH_STANDALONE_INFO_TERMINAL_INDEX]) UInt256];
        BOOL terminalHardened = [((NSNumber *)infoDictionary[DERIVATION_PATH_STANDALONE_INFO_TERMINAL_HARDENED]) boolValue];
        UInt256 indexes[] = {terminalIndex};
        BOOL hardenedIndexes[] = {terminalHardened};
        if (!(self = [self initWithIndexes:indexes hardened:hardenedIndexes length:0 type:DSDerivationPathType_ViewOnlyFunds signingAlgorithm:DSKeyType_ECDSA reference:DSDerivationPathReference_Unknown onChain:chain])) return nil;
        _walletBasedExtendedPublicKeyLocationString = extendedPublicKeyIdentifier;
        NSData *data = getKeychainData([DSDerivationPath standaloneExtendedPublicKeyLocationStringForUniqueID:extendedPublicKeyIdentifier], &error);
        if (error) return nil;
        _extendedPublicKey = [DSKey keyWithExtendedPublicKeyData:data forKeyType:DSKeyType_ECDSA];

        _depth = infoDictionary[DERIVATION_PATH_STANDALONE_INFO_DEPTH];

        [self loadAddresses];

    }

    pub fn init_with_indexes(
        indexes: Vec<UInt256>,
        hardened_indexes: Vec<bool>,
        r#type: derivation_paths::Type,
        signing_algorithm: keys::Type,
        reference: Reference,
        chain: Chain<P>
    ) -> Self<P> {
        // chainContext
        Self {
            indexes,
            hardened_indexes,
            extended_public_key: None,
            r#type,
            signing_algorithm,
            chain,
            account: None,
            wallet: None,
            string_representation: None,
            standalone_extended_public_key_unique_id: None,
            wallet_based_extended_public_key_location_string: None,
            wallet_based_extended_private_key_location_string: None,
            balance: 0,
            sync_block_height: u32::MAX,
            all_addresses: HashSet::new(),
            used_addresses: HashSet::new(),
            reference,
            derivation_path_is_known: true,
            addresses_loaded: false,
            depth: 0
        }
    }

    // MARK: - Key Generation

    pub fn generate_extended_public_key_from_seed(&self, seed: Option<Vec<u8>>, wallet_unique_id: Option<String>) -> Option<dyn IKey<P>> {
        //return [self generate_extended_public_key_from_seed:seed storeUnderWalletUniqueId:walletUniqueId storePrivateKey:NO];
        self.generate_extended_public_key_from_seed(seed, wallet_unique_id, false)
    }

    pub fn generate_extended_public_key_from_seed_and_store_private_key(&mut self, seed: Option<Vec<u8>>, wallet_unique_id: Option<String>, store_private_key: bool) -> Option<dyn IKey<P>> {
        if seed.is_none() || (self.indexes.len() == 0 && self.reference != Reference::Root) { // there needs to be at least 1 length
            return None;
        }
        let seed_key = Key::key_with_seed_data(seed, self.signing_algorithm);
        if seed_key.is_none() {
            return None;
        }
        let ext_pub_key = seed_key.unwrap().private_derive_to_256_bit_derivation_path(*self);
        self.extended_public_key = ext_pub_key;
        if self.extended_public_key.is_none() {
            assert!(false, "extendedPublicKey should be set");
            return None
        }

        if let Some(wallet_id) = wallet_unique_id {
            //setKeychainData(self.extendedPublicKey.extendedPublicKeyData, [self walletBasedExtendedPublicKeyLocationStringForWalletUniqueID:walletUniqueId], NO);
            // if store_private_key {
            //     setKeychainData(_extendedPublicKey.extendedPrivateKeyData, [self walletBasedExtendedPrivateKeyLocationStringForWalletUniqueID:walletUniqueId], YES);
            // }
        }
        self.extended_public_key.unwrap().forget_private_key();
        self.extended_public_key
    }


    pub fn deserialized_extended_private_key(extended_private_key_string: &str, chain: Chain) -> Option<Vec<u8>> {

        let mut depth: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
        let mut fingerprint: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
        let mut chain_hash: std::mem::MaybeUninit<UInt256> = std::mem::MaybeUninit::uninit();
        let mut child: std::mem::MaybeUninit<UInt256> = std::mem::MaybeUninit::uninit();
        let mut hardened: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
        let mut privkey: std::mem::MaybeUninit<Vec<u8>> = std::mem::MaybeUninit::uninit();

        let is_valid = unsafe { deserialize(
            extended_private_key_string,
            depth.as_mut_ptr(),
            fingerprint.as_mut_ptr(),
            hardened.as_mut_ptr(),
            child.as_mut_ptr(),
            chain_hash.as_mut_ptr(),
            privkey.as_mut_ptr(),
            chain.r#type == MainNet) };

        if is_valid {
            let offset = &mut 0;
            // NSMutableData *masterPrivateKey = [NSMutableData secureData];
            let mut master_private_key: Vec<u8> = Vec::new();
            master_private_key.write_with::<u32>(offset, unsafe { fingerprint.assume_init() }, byte::LE).unwrap();
            master_private_key.write_with::<UInt256>(offset, unsafe { chain_hash.assume_init() }, byte::LE).unwrap();
            master_private_key.write_with(offset, unsafe { privkey.assume_init() }, Default::default()).unwrap(); // 4
            return Some(master_private_key);
        }
        None
    }


pub fn deserialized_extended_public_key<'a>(
        extended_public_key_string: &'a str,
        chain: Chain,
        depth: *mut u8,
        terminal_hardened: *mut bool,
        terminal_index: *mut UInt256,
    ) -> Vec<u8> {

    }

    + (NSData *)deserializedExtendedPublicKey:(NSString *)extendedPublicKeyString onChain:(DSChain *)chain rDepth:(uint8_t *)depth rTerminalHardened:(BOOL *)terminalHardened rTerminalIndex:(UInt256 *)terminalIndex {
    uint32_t fingerprint;
    UInt256 chainHash;
    NSData *pubkey = nil;
    NSMutableData *masterPublicKey = [NSMutableData secureData];
    BOOL valid = deserialize(extendedPublicKeyString, depth, &fingerprint, terminalHardened, terminalIndex, &chainHash, &pubkey, [chain isMainnet]);
    if (!valid) return nil;
    [masterPublicKey appendUInt32:fingerprint];
    [masterPublicKey appendBytes:&chainHash length:32];
    [masterPublicKey appendData:pubkey];
    return [masterPublicKey copy];
}




}

