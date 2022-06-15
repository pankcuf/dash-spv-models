use std::collections::HashMap;
use dash_spv_primitives::crypto::byte_util::Zeroable;
use dash_spv_primitives::crypto::UInt256;
use crate::chain::chain_parameters::ChainParameters;
use crate::chain::checkpoint::Checkpoint;
use crate::common::{Block, ChainType, LLMQType};
use crate::derivation_paths::funds_path::FundsPath;
use crate::derivation_paths::Path;
use crate::derivation_paths::path::DerivationPath;
use crate::identity::identity::Identity;
use crate::tx::direction::Direction;
use crate::tx::Transaction;
use crate::tx::transaction::ITransaction;
use crate::wallet::account::Account;
use crate::wallet::wallet::Wallet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Chain<P: ChainParameters> {
    pub parameters: P,

    pub r#type: ChainType,
    pub orphans: HashMap<UInt256, Block>,
    pub terminal_blocks: HashMap<UInt256, Block>,

    /// The chain manager is a container for all managers (peer, identity, governance, masternode,
    /// spork and transition). It also is used to control the sync process.
    // @property (nonatomic, weak, nullable) DSChainManager *chainManager;

    /// @brief The chain entity associated in Core Data in the required context.
    // - (DSChainEntity *)chainEntityInContext:(NSManagedObjectContext *)context;

    /// The managed object context of the chain.
    // @property (nonatomic, readonly) NSManagedObjectContext *chainManagedObjectContext;

// MARK: - L1 Network Chain Info

    /// An array of known hard coded checkpoints for the chain.
    pub checkpoints: Vec<Checkpoint>,

    /// MARK: Sync

    /// The genesis hash is the hash of the first block of the chain. For a devnet this is actually
    /// the second block as the first block is created in a special way for devnets.
    pub genesis_hash: UInt256,

    /// The magic number is used in message headers to indicate what network (or chain) a
    /// message is intended for.
    pub magic_number: u32,

    /// The base reward is the intial mining reward at genesis for the chain. This goes down by 7%
    /// every year. A SPV client does not validate that the reward amount is correct as it would
    /// not make sense for miners to enter incorrect rewards as the blocks would be rejected by
    /// full nodes.
    pub base_reward: u64,

    // TODO: Migrate to ChainParameters
    /// min_protocol_version is the minimum protocol version that peers on this chain can
    /// communicate with. This should only be changed in the case of devnets.
    pub min_protocol_version: u32,

    // TODO: Migrate to ChainParameters
    /// protocol_version is the protocol version that we currently use for this chain. This should
    /// only be changed in the case of devnets.
    pub protocol_version: u32,

    /// headers_max_amount is the maximum amount of headers that is expected from peers.
    pub headers_max_amount: u32,

    /// max_proof_of_work is the lowest amount of work effort required to mine a block on the chain.
    pub max_proof_of_work: UInt256,

    /// max_proof_of_work_target is the lowest amount of work effort required to mine a block on the
    /// chain. Here it is represented as the compact target.  */
    pub max_proof_of_work_target: u32,

    /// allow_min_difficulty_blocks is set to TRUE on networks where mining is low enough that it can
    /// be attacked by increasing difficulty with ASICs and then no longer running ASICs. This is
    /// set to NO for Mainnet, and generally should be YES on all other networks.
    pub allow_min_difficulty_blocks: bool,

    /// This is the minimum amount that can be entered into an amount for a output for it not to be
    /// considered dust.
    pub min_output_amount: u64,

// MARK: Fees

    pub fee_per_byte: u64,

    /// The fee for transactions in L1 are now entirely dependent on their size.

// MARK: Ports

    /// The standard port for the chain for L1 communication.
    pub standard_port: u32,

    /// The standard port for the chain for L2 communication through JRPC.
    pub standard_dapi_jrpc_port: u32,

    /// The standard port for the chain for L2 communication through GRPC.
    pub standard_dapi_grpc_port: u32,

// MARK: Sporks

    /// The spork public key as a hex string.
    pub spork_public_key_hex_string: Option<String>,

    /// The spork private key as a base 58 string.
    pub spork_private_key_base58string: Option<String>,

    /// The spork address base 58 string (addresses are known to be base 58).
    pub spork_address: Option<String>,

// MARK: - L2 Network Chain Info

    /// platform_protocol_version is the protocol version that we currently use for the platform chain.
    /// This should only be changed in the case of devnets.
   pub platform_protocol_version: u32,

    /// The dpns contract id.
    pub dpns_contract_id: UInt256,

    /// The dashpay contract id.
    pub dashpay_contract_id: UInt256,

// MARK: - DashSync Chain Info

    /// The chain type (MainNet, TestNet or DevNet).
    pub chain_type: ChainType,

    /// A threshold after which a peer will be banned.
    pub peer_misbehaving_threshold: u32,

    /// True if this chain syncs the blockchain. All Chains currently sync the blockchain.
    pub syncs_blockchain: bool,

    /// True if this chain should sync headers first for masternode list verification.
    pub needs_initial_terminal_headers_sync: bool,

    /// The default transaction version used when sending transactions.
    pub transaction_version: u16,

    /*! @brief The number of minimum_difficulty_blocks.  */
    pub minimum_difficulty_blocks: u32,

    /*! @brief The type of quorum used for Instant Send Locks.  */
    pub quorum_type_for_is_locks: LLMQType,

    /*! @brief The type of quorum used for Chain Locks.  */
    pub quorum_type_for_chain_locks: LLMQType,

    /*! @brief The type of quorum used for Platform.  */
    pub quorum_type_for_platform: LLMQType,


// MARK: Names and Identifiers

    /*! @brief The unique identifier of the chain. This unique id follows the same chain accross devices because it is the short hex string of the genesis hash.  */
    pub unique_id: String,


    // /*! @brief The localized name of the chain (Mainnet-Testnet-Devnet).  */
    // @property (nonatomic, readonly) NSString *localizedName;

    // - (void)setDevnetNetworkName:(NSString *)networkName;

// MARK: - Wallets

    /*! @brief The wallets in the chain.  */
    pub wallets: Vec<Wallet>,

    /*! @brief Conveniance method. Does this walleet have a chain?  */
    pub has_awallet: bool,

    /*! @brief Conveniance method. The earliest known creation time for any wallet in this chain.  */
    pub earliest_wallet_creation_time: f64,


// MARK: - Standalone Derivation Paths

    /*! @brief Standalone derivation paths used in this chain. This is currently an experimental feature  */
    pub standalone_derivation_paths: Vec<DerivationPath>,

    /*! @brief Conveniance method to find out if the chain has a standalone derivation path. Standalone derivation paths are currently an experimental feature  */
    pub has_astandalone_derivation_path: bool,



// MARK: - Blocks and Headers

    /*! @brief The last known chain sync block on the chain.  */
    pub last_sync_block: Option<Block>,

    /*! @brief The last known chain sync block on the chain, don't recover from checkpoints if it is not known.  */
    pub last_sync_block_dont_use_checkpoints: Option<Block>,

    /*! @brief The last known terminal block on the chain.  */
    pub last_terminal_block: Option<Block>,


    /*! @brief The last known orphan on the chain. An orphan is a block who's parent is currently not known.  */
    pub last_orphan: Option<Block>,

    /*! @brief A dictionary of the the most recent known blocks keyed by block hash.  */
    pub recent_blocks: HashMap<UInt256, MerkleBlock>,

    /*! @brief A short hex string of the last block's block hash.  */
    pub chain_tip: Option<String>,

    /*! @brief The block locator array is an array of the 10 most recent block hashes in decending order followed by block hashes that double the step back each iteration in decending order and finishing with the previous known checkpoint after that last hash. Something like (top, -1, -2, -3, -4, -5, -6, -7, -8, -9, -11, -15, -23, -39, -71, -135, ..., 0).  */
    pub chain_sync_block_locator_array: Vec<UInt256>,


// MARK: Chain Sync

    /*! @brief Returns the hash of the last persisted sync block. The sync block itself most likely is not persisted.  */
    pub last_persisted_chain_sync_block_hash: UInt256,

    /*! @brief Returns the height of the last persisted sync block. The sync block itself most likely is not persisted.  */
    pub last_persisted_chain_sync_block_height: u32,

    /*! @brief Returns the timestamp of the last persisted sync block. The sync block itself most likely is not persisted.  */
    pub last_persisted_chain_sync_block_timestamp: f64,

    /*! @brief Returns the locators of the last persisted chain sync block. The sync block itself most likely is not persisted.  */
    pub last_persisted_chain_sync_locators: Vec<UInt256>,

// MARK: Last Block Information

    /*! @brief Returns the height of the last sync block.  */
    pub last_sync_block_height: u32,

    /*! @brief Returns the hash of the last sync block.  */
    pub last_sync_block_hash: UInt256,

    /*! @brief Returns the timestamp of the last sync block.  */
    pub last_sync_block_timestamp: f64,

    /*! @brief Returns the height of the last header used in initial headers sync to get the deterministic masternode list.  */
    pub last_terminal_block_height: u32,

    /*! @brief Returns the height of the best block.  */
    pub best_block_height: u32,

    /*! @brief Returns the estimated height of chain. This is reported by the current download peer but can not be verified and is not secure.  */
    pub estimated_block_height: u32,


// MARK: Chain Lock

    /*! @brief Returns the last chainLock known by the chain at the heighest height.  */
    pub last_chain_lock: ChainLock,


// MARK: - Transactions

    /*! @brief Returns all wallet transactions sorted by date, most recent first.  */
    pub all_transactions: Vec<dyn ITransaction>,


// MARK: - Bloom Filter

    /*! @brief Returns if a filter can be created for the chain. Generally this means that the chain has at least a wallet or a standalone derivation path. */
    pub can_construct_a_filter: bool,


// MARK: - Accounts and Balances

    /*! @brief The current wallet balance excluding transactions known to be invalid.  */
    pub balance: u64,


// MARK: - Governance

    /*! @brief Returns a count of all governance objects.  */
    pub total_governance_objects_count: u32,

// MARK: - Identities

    /*! @brief Returns a count of local blockchain identities.  */
    pub local_blockchain_identities_count: u32,

    /*! @brief Returns a count of blockchain invitations that have been created locally.  */
    pub local_blockchain_invitations_count: u32,

    /*! @brief Returns an array of all local blockchain identities.  */
    pub local_blockchain_identities: Vec<Identity>,

    /*! @brief Returns a dictionary of all local blockchain identities keyed by uniqueId.  */
    pub local_blockchain_identities_by_unique_id_dictionary: HashMap<UInt256, Identity>,


    checkpoints_by_hash_dictionary: HashMap<UInt256, Checkpoint>,
    checkpoints_by_height_dictionary: HashMap<u32, Checkpoint>,


    viewing_account: Account<P>,
    estimatedBlockHeights: HashMap<i32, Vec<Peer>>,
    cached_minimum_difficulty_blocks: u32,
    best_estimated_block_height: u32,


    transactionHashHeights: HashMap<UInt256, u32>,
    transactionHashTimestamps: HashMap<UInt256, f64>,

    terminalHeadersOverrideUseCheckpoint: Checkpoint,
    syncHeadersOverrideUseCheckpoint: Checkpoint,
    lastCheckpoint: Checkpoint,

    lastNotifiedBlockDidChange: f64,


    is_transient: bool,
    chain_wallets_key: String,

    @property (nonatomic, assign) uint32_t cachedMinProtocolVersion;
    @property (nonatomic, assign) uint32_t cachedProtocolVersion;
    @property (nonatomic, assign) UInt256 cachedMaxProofOfWork;
    @property (nonatomic, assign) uint32_t cachedStandardPort;
    @property (nonatomic, assign) uint32_t cachedStandardDapiJRPCPort;
    @property (nonatomic, assign) uint32_t cachedStandardDapiGRPCPort;
    @property (nonatomic, assign) UInt256 cachedDpnsContractID;
    @property (nonatomic, assign) UInt256 cachedDashpayContractID;

    // @property (nonatomic, strong) NSManagedObjectContext *chainManagedObjectContext;
    // @property (nonatomic, strong) NSTimer *lastNotifiedBlockDidChangeTimer;


}

impl<P> Chain<P> {

    pub fn new() -> Chain<P> {
        //assert!([NSThread isMainThread], @"Chains should only be created on main thread (for chain entity optimizations)");
        self.or = [NSMutableDictionary dictionary];
        self.mWallets = [NSMutableArray array];
        self.estimatedBlockHeights = [NSMutableDictionary dictionary];

        self.transactionHashHeights = [NSMutableDictionary dictionary];
        self.transactionHashTimestamps = [NSMutableDictionary dictionary];

        self.lastNotifiedBlockDidChange = 0;

        if (self.checkpoints) {
            self.genesisHash = self.checkpoints[0].blockHash;
            dispatch_sync(self.networkingQueue, ^{
                self.chainManagedObjectContext = [NSManagedObjectContext chainContext];
            });
        }

        self.feePerByte = DEFAULT_FEE_PER_B;
        uint64_t feePerByte = [[NSUserDefaults standardUserDefaults] doubleForKey:FEE_PER_BYTE_KEY];
        if (feePerByte >= MIN_FEE_PER_B && feePerByte <= MAX_FEE_PER_B) self.feePerByte = feePerByte;

        Chain {
            r#type: ChainType::MainNet,
            orphans: Default::default(),
            terminal_blocks: Default::default(),
            checkpoints_by_hash_dictionary: Default::default(),
            checkpoints_by_height_dictionary: Default::default(),
            last_sync_block: Block {},
            last_terminal_block: Block {},
            last_orphan: Block {},
            checkpoints: vec![]
        }

    }

    pub fn fee_for_tx_size(&self, size: usize) -> u64 {

    }

    /*! @brief Whether chain should process this type of quorum.  */
    pub fn should_process_quorum_of_type(llmq_type: LLMQType) -> bool {

    }

    /*! @brief Returns all standard derivaton paths used for the chain based on the account number.  */
    pub fn standard_derivation_paths_for_account_number<DP: DerivationPath<P>>(&self, account_number: u32) -> Vec<DP> {
        if account_number == 0 {
            vec![
                FundsPath::bip32_derivation_path_for_account_number(account_number, self),
                FundsPath::bip44_derivation_path_for_account_number(account_number, self),
                Path::master_blockchain_identity_contacts_derivation_path_for_account_number(account_number, self)
            ]
        } else {
            vec![
                // don't include BIP32 derivation path on higher accounts
                FundsPath::bip44_derivation_path_for_account_number(account_number, self),
                Path::master_blockchain_identity_contacts_derivation_path_for_account_number(account_number, self)
            ]
        }
    }

    /*! @brief Unregister a wallet from the chain, it will no longer be loaded or used.  */
    pub fn unregister_wallet(&self, wallet: Wallet) {

    }

    /*! @brief Register a wallet to the chain.  */
    pub fn register_wallet(&self, wallet: Wallet) {

    }

    /*! @brief Unregister all wallets from the chain, they will no longer be loaded or used.  */
    pub fn unregister_all_wallets(&self) {

    }

    /*! @brief Unregister all wallets from the chain that don't have an extended public key in one of their derivation paths, they will no longer be loaded or used.  */
    pub fn unregister_all_wallets_missing_extended_public_keys(&self) {

    }

    /*! @brief Unregister a standalone derivation path from the chain, it will no longer be loaded or used. Standalone derivation paths are currently an experimental feature  */
    pub fn unregister_standalone_derivation_path(&self, derivation_path: DerivationPath) {

    }

    /*! @brief Register a standalone derivation path to the chain. Standalone derivation paths are currently an experimental feature  */
    pub fn register_standalone_derivation_path(&self, derivation_path: DerivationPath) {

    }

    // MARK: - Checkpoints

    /*! @brief Returns the last checkpoint that has a masternode list attached to it.  */
    pub fn last_checkpoint_having_masternode_list(&self) -> Option<Checkpoint> {

    }

    /*! @brief Returns the checkpoint matching the parameter block hash, if one exists.  */
    pub fn checkpoint_for_block_hash(&self, block_hash: UInt256) -> Option<Checkpoint> {

    }

    /*! @brief Returns the checkpoint at a given block height, if one exists at that block height.  */
    pub fn checkpoint_for_block_height(&self, block_height: u32) -> Option<Checkpoint> {

    }

    /*! @brief Returns the last checkpoint on or before the given height.  */
    pub fn last_checkpoint_on_or_before_height(&self, height: u32) -> Checkpoint {

    }

    /*! @brief Returns the last checkpoint on or before the given timestamp.  */
    pub fn last_checkpoint_on_or_before_timestamp(&self, timestamp: f64) -> Checkpoint {

    }

    /*! @brief When used this will change the checkpoint used for initial headers sync. This value is not persisted.  */
    pub fn use_checkpoint_before_or_on_height_for_terminal_blocks_sync(&self, block_height: u32) {

    }

    /*! @brief When used this will change the checkpoint used for main chain syncing. This value is not persisted.  */
    pub fn use_checkpoint_before_or_on_height_for_syncing_chain_blocks(&self, block_height: u32) {

    }



    /*! @brief The last known block on the chain before the given timestamp.  */
    pub fn last_chain_sync_block_on_or_before_timestamp(&self, timestamp: f64) -> Block {

    }

    /*! @brief The last known block or header on the chain before the given timestamp.  */
    pub fn last_block_on_or_before_timestamp(&self, timestamp: f64) -> Block {

    }

    /*! @brief This block locator array is an array of 10 block hashes in decending order before the given timestamp followed by block hashes that double the step back each iteration in decending order and finishing with the previous known checkpoint after that last hash. Something like (top, -1, -2, -3, -4, -5, -6, -7, -8, -9, -11, -15, -23, -39, -71, -135, ..., 0).  */
    pub fn block_locator_array_on_or_before_timestamp(&self, timestamp: f64, includeInitialTerminalBlocks: bool) -> Vec<UInt256> {

    }
    /*! @brief The timestamp of a block at a given height.  */
    // seconds since 1970, 00:00:00 01/01/01 GMT
    pub fn timestamp_for_block_height(&self, block_height: u32) -> f64 {

    }

    /*! @brief The block on the main chain at a certain height. By main chain it is understood to mean not forked chain - this could be on mainnet, testnet or a devnet.  */
    pub fn block_at_height(&self, height: u32) -> Option<MerkleBlock> {

    }

    /*! @brief Returns a known block with the given block hash. This does not have to be in the main chain. A null result could mean that the block was old and has since been discarded.  */
    pub fn block_for_block_hash(&self, block_hash: UInt256) -> Option<MerkleBlock> {

    }

    /*! @brief Returns a known block in the main chain with the given block hash. A null result could mean that the block was old and has since been discarded.  */
    pub fn recent_terminal_block_for_block_hash(&self, block_hash: UInt256) -> Option<MerkleBlock> {

    }

    /*! @brief Returns a known block with a given distance from the chain tip. A null result would mean that the given distance exceeded the number of blocks kept locally.  */
    pub fn block_from_chain_tip(&self, blocksAgo: u32) -> Option<MerkleBlock> {

    }

    /*! @brief Returns the height of a block having the given hash. If no block is found returns UINT32_MAX  */
    pub fn height_for_block_hash(&self, block_hash: UInt256) -> u32 {

    }

    /*! @brief Returns the height of a block having the given hash. This does less expensive checks than height_for_block_hash and is not garanteed to be accurate, but can be used for logging. If no block is found returns UINT32_MAX  */
    pub fn quick_height_for_block_hash(&self, block_hash: UInt256) -> u32 {

    }

    /*! @brief Adds a chainLock to the chain and applies it corresponding block. It will be applied to both terminal blocks and sync blocks.  */
    pub fn add_chain_lock(chain_lock: DSChainLock) -> bool {

    }

    /*! @brief Returns if there is a block at the following height that is confirmed.  */
    pub fn block_height_chain_locked(height: u32) -> bool {

    }


    /*! @brief Returns the transaction with the given hash if it's been registered in any wallet on the chain (might also return non-registered) */
    pub fn transaction_for_hash<'a>(hash: UInt256) -> Option<Transaction<'a>> {

    }


    /*! @brief Returns the direction of a transaction for the chain (Sent - Received - Moved - Not Account Funds) */
    pub fn direction_of_transaction<'a>(&self, transaction: Transaction<'a>) -> Option<Direction> {

    }

    /*! @brief Returns the amount received globally from the transaction (total outputs to change and/or receive addresses) */
    pub fn amount_received_from_transaction<'a>(&self, transaction: Transaction<'a>) -> u64 {

    }

    /*! @brief Returns the amount sent globally by the trasaction (total wallet outputs consumed, change and fee included) */
    pub fn amount_sent_by_transaction<'a>(&self, transaction: Transaction<'a>) -> u64 {

    }

    /*! @brief Returns if this transaction has any local references. Local references are a pubkey hash contained in a wallet, pubkeys in wallets special derivation paths, or anything that would make the transaction relevant for this device. */
    pub fn transaction_has_local_references<'a>(&self, transaction: Transaction<'a>) -> bool {

    }

    /*! @brief Returns a bloom filter with the given false positive rate tweaked with the value tweak. The value tweak is generally peer specific. */
    pub fn bloom_filter_with_false_positive_rate(&self, false_positive_rate: f32, tweak: u32) -> BloomFilter {

    }

    /*! @brief All accounts that contain the specified transaction hash. The transaction is also returned if it is found.  */
    pub fn accounts_for_transaction_hash<'a>(&self, hash: UInt256, transaction: Option<Transaction<'a>>) -> Vec<Account> {

    }

    /*! @brief Returns the first account with a balance.   */
    - (DSAccount *_Nullable)firstAccountWithBalance;

    /*! @brief Returns an account to which the given transaction is or can be associated with (even if it hasn't been registered), no account if the transaction is not associated with the wallet.  */
    - (DSAccount *_Nullable)firstAccountThatCanContainTransaction:(DSTransaction *)transaction;

    /*! @brief Returns all accounts to which the given transaction is or can be associated with (even if it hasn't been registered)  */
    - (NSArray *)accountsThatCanContainTransaction:(DSTransaction *_Nonnull)transaction;

    /*! @brief Returns an account to which the given transaction hash is associated with, no account if the transaction hash is not associated with the wallet.  */
    - (DSAccount *_Nullable)firstAccountForTransactionHash:(UInt256)txHash transaction:(DSTransaction *_Nullable *_Nullable)transaction wallet:(DSWallet *_Nullable *_Nullable)wallet;

    /*! @brief Returns an account to which the given address is contained in a derivation path.  */
    - (DSAccount *_Nullable)accountContainingAddress:(NSString *)address;

    /*! @brief Returns an account to which the given address is known by a dashpay outgoing derivation path.  */
    - (DSAccount *_Nullable)accountContainingDashpayExternalDerivationPathAddress:(NSString *)address;




//     - (DSBlockchainIdentity *)blockchainIdentityForUniqueId:(UInt256)uniqueId {
//     NSAssert(uint256_is_not_zero(uniqueId), @"uniqueId must not be null");
//     return [self blockchainIdentityForUniqueId:uniqueId foundInWallet:nil includeForeignBlockchainIdentities:NO];
// }

    pub fn blockchain_identity_for_unique_id(&self, unique_id: UInt256) -> Option<&Identity> {
        assert!(!unique_id.is_zero(), "uniqueId must not be null");
        self.blockchain_identity_for_unique_id_in_wallet(unique_id, None, false)
    }

    pub fn blockchain_identity_for_unique_id_in_wallet(&self, unique_id: UInt256, &mut found_in_wallet: Option<Wallet<P>>, include_foreign_blockchain_identities: bool) -> Option<&Identity> {
        assert!(!unique_id.is_zero(), "uniqueId must not be null");
        let mut identity: Option<&Identity> = None;
        self.wallets.iter().for_each(|&wallet| {
            if let Some(found) = wallet.blockchain_identity_for_unique_id(unique_id) {
                if let Some(found_in) = found_in_wallet {
                    *found_in_wallet = wallet;
                }
                identity = Some(found);
            }
        });
        if identity.is_some() {
            return identity;
        }
        if include_foreign_blockchain_identities {
            // return [self.chainManager.identitiesManager foreignBlockchainIdentityWithUniqueId:uniqueId];
            None
        } else {
            None
        }
    }


}

// static ONCE: Once = Once::new();
// let mut num = 0;
// ONCE.call_once(|| num += 1);
// ONCE.call_once(|| num += 1);
