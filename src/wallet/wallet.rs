use std::collections::{HashMap, HashSet};
use dash_spv_primitives::crypto::byte_util::Zeroable;
use dash_spv_primitives::crypto::UInt256;
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::identity::identity::Identity;
use crate::identity::invitation::Invitation;
use crate::tx::transaction::ITransaction;
use crate::tx::UTXO;
use crate::wallet::account::Account;
use crate::wallet::special_transaction_wallet_holder::SpecialTransactionWalletHolder;


pub const DUFFS: u64 = 100000000;
pub const MAX_MONEY: u64 = 21000000 * DUFFS;

#[derive(Clone)]
pub struct Wallet<P: ChainParameters> {
    pub ordered_accounts: HashMap<u64, Account<P>>,
    pub last_account_number: u32,
    pub accounts: Vec<Account<P>>,
    pub special_transactions_holder: SpecialTransactionWalletHolder,
    pub blockchain_identities: HashMap<UInt256, Identity>,
    pub blockchain_invitations: HashMap<UInt256, Invitation>,
    pub default_blockchain_identity: Option<Identity>,
    pub blockchain_identity_addresses: Option<Vec<String>>,
    pub provider_owner_addresses: Option<Vec<String>>,
    pub provider_voting_addresses: Option<Vec<String>>,
    pub provider_operator_addresses: Option<Vec<String>>,

    // This is unique among all wallets and all chains
    pub unique_id_string: Option<String>,

    pub wallet_creation_time: Option<f64>,

    // set to true if this wallet is not stored on disk
    pub is_transient: bool,

    // chain for the wallet
    pub chain: Chain<P>,

    // current wallet balance excluding transactions known to be invalid
    pub balance: u64,

    // all previously generated external addresses
    pub all_receive_addresses: HashSet<String>,

    // all previously generated internal addresses
    pub all_change_addresses: HashSet<String>,

    // NSValue objects containing UTXO structs
    pub unspent_outputs: Vec<UTXO>,

    // latest 100 transactions sorted by date, most recent first
    pub recent_transactions: Vec<dyn ITransaction>,

    // all wallet transactions sorted by date, most recent first
    pub all_transactions: Vec<dyn ITransaction>,

    // the total amount spent from the wallet (excluding change)
    pub total_sent: u64,

    // the total amount received by the wallet (excluding change)
    pub total_received: u64,

    // the first unused index for blockchain identity registration funding
    pub unused_blockchain_identity_index: u32,

    // the first unused index for invitations
    pub unused_blockchain_invitation_index: u32,

    // the amount of known blockchain identities
    pub blockchain_identities_count: u32,

    // the amount of known blockchain invitations
    pub blockchain_invitations_count: u32,

    // The fingerprint for currentTransactions
    pub chain_synchronization_fingerprint: Vec<u8>,


}

impl<P> Wallet<P> {
    pub fn blockchain_identity_for_unique_id(&self, unique_id: UInt256) -> Option<&Identity> {
        assert!(!unique_id.is_zero(), "uniqueId must not be null");
        self.blockchain_identities.values().find(|&identity| identity.unique_id == unique_id)
    }

}
