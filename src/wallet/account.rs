use std::collections::{HashMap, HashSet};
use dash_spv_primitives::crypto::UInt256;
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::derivation_paths;
use crate::derivation_paths::funds_path::FundsPath;
use crate::derivation_paths::incoming_funds_path::IncomingFundsPath;
use crate::derivation_paths::{Path, Reference};
use crate::derivation_paths::path::DerivationPath;
use crate::tx::{CoinbaseTransaction, Transaction, UTXO};
use crate::tx::transaction::ITransaction;
use crate::wallet::wallet::Wallet;

pub struct Account<P: ChainParameters> {
    //@property (nonatomic, readonly) NSManagedObjectContext *managedObjectContext;

    // BIP 43 derivation paths
    pub fund_derivation_paths: Option<Vec<dyn DerivationPath<P>>>,
    pub outgoing_fund_derivation_paths: Option<Vec<dyn DerivationPath<P>>>,
    pub default_derivation_path: Option<dyn DerivationPath<P>>,
    pub bip44derivation_path: Option<FundsPath<P>>,
    pub bip32derivation_path: Option<FundsPath<P>>,
    pub master_contacts_derivation_path: Option<dyn DerivationPath<P>>,

    pub wallet: Option<Wallet<P>>, //weak

    pub unique_id: String,
    pub account_number: u32,

    /// current wallet balance excluding transactions known to be invalid
    pub balance: u64,

    pub unspent_outputs: Vec<UTXO>,

    /// latest 100 transactions sorted by date, most recent first
    pub recent_transactions: Vec<dyn ITransaction>,

    /// latest 100 transactions sorted by date, most recent first
    pub recent_transactions_with_internal_output: Vec<dyn ITransaction>,

    /// all wallet transactions sorted by date, most recent first
    pub all_transactions: Vec<dyn ITransaction>,

    /// all wallet transactions sorted by date, most recent first
    pub coinbase_transactions: Vec<CoinbaseTransaction>,


    /// Does this account have any coinbase rewards
    pub has_coinbase_transaction: bool,

    /// returns the first unused external address
    pub receive_address: Option<String>,

    /// returns the first unused internal address
    pub change_address: Option<String>,

    /// all previously generated external addresses
    pub external_addresses: Vec<String>,

    /// all previously generated internal addresses
    pub internal_addresses: Vec<String>,

    /// all the contacts for an account
    pub contacts: Vec<PotentialOneWayFriendship>,

    /// has an extended public key missing in one of the account derivation paths
    pub has_an_extended_public_key_missing: bool,

    // BIP 43 derivation paths
    contact_incoming_fund_derivation_paths: HashMap<UInt256, IncomingFundsPath<P>>,
    contact_outgoing_fund_derivation_paths: HashMap<UInt256, IncomingFundsPath<P>>,

    balance_history: Vec<u64>,
    spent_outputs: HashSet<UTXO>,
    invalid_transaction_hashes: HashSet<UInt256>,
    pending_transaction_hashes: HashSet<UInt256>,


    pending_coinbase_locked_transaction_hashes: HashMap<u32, HashSet<UInt256>>,
    transactions: HashSet<dyn ITransaction>,
    transactions_to_save: Vec<dyn ITransaction>,
    transactions_to_save_in_block_save: HashMap<u32, Vec<dyn ITransaction>>,
    utxos: HashSet<UTXO>,
    all_tx: HashMap<UInt256, dyn ITransaction>,

    // the total amount spent from the account (excluding change)
    total_sent: u64,

    // the total amount received to the account (excluding change)
    total_received: u64,

    is_view_only_account: bool,

    first_transaction_hash: UInt256,
}

impl<P> Account<P> {

    pub fn init_with_account_number(account_number: u32, derivation_paths:Vec<FundsPath<P>) -> Self<P> {
        assert!(derivation_paths);

        let mut this = Self {
            fund_derivation_paths: Some(vec![]),
            outgoing_fund_derivation_paths: None,
            default_derivation_path: None,
            bip44derivation_path: None,
            bip32derivation_path: None,
            master_contacts_derivation_path: None,
            wallet: None,
            unique_id: "".to_string(),
            account_number,
            balance: 0,
            unspent_outputs: vec![],
            recent_transactions: vec![],
            recent_transactions_with_internal_output: vec![],
            all_transactions: vec![],
            coinbase_transactions: vec![],
            has_coinbase_transaction: false,
            receive_address: None,
            change_address: None,
            external_addresses: vec![],
            internal_addresses: vec![],
            contacts: vec![],
            has_an_extended_public_key_missing: false,
            contact_incoming_fund_derivation_paths: HashMap::new(),
            contact_outgoing_fund_derivation_paths: HashMap::new(),
            balance_history: vec![],
            spent_outputs: Default::default(),
            invalid_transaction_hashes: Default::default(),
            pending_transaction_hashes: Default::default(),
            pending_coinbase_locked_transaction_hashes: Default::default(),
            transactions: (),
            transactions_to_save: vec![],
            transactions_to_save_in_block_save: Default::default(),
            utxos: Default::default(),
            all_tx: (),
            total_sent: 0,
            total_received: 0,
            is_view_only_account: false,
            first_transaction_hash: Default::default()
        };

        this.verify_and_assign_added_derivation_paths(derivation_paths);

        derivation_paths.iter().filter_map(|&path| type_ path::)

    for (DSDerivationPath *derivationPath in derivationPaths) {
        if ([derivationPath isKindOfClass:[DSFundsDerivationPath class]]) {
            [self.mFundDerivationPaths addObject:(DSFundsDerivationPath *)derivationPath];
        }
        derivationPath.account = self;
    }
    self.transactions = [NSMutableOrderedSet orderedSet];
    self.allTx = [NSMutableDictionary dictionary];
    self.managedObjectContext = context ? context : [NSManagedObjectContext chainContext];
    self.transactionsToSave = [NSMutableArray array];
    self.transactionsToSaveInBlockSave = [NSMutableDictionary dictionary];
    self.isViewOnlyAccount = FALSE;
    return self;
}


    pub fn account_with_account_number(account_number: u32, derivation_paths: Vec<derivation_paths::FundsPath>) {
        Account::initWithAccountNumber(account_number, derivation_paths)
    }

    pub fn standard_accounts_to_account_number(account_number: u32, chain: Chain<P>) -> Vec<Account<P>> {
        (0..=account_number).into_iter().map(|i| Self::account_with_account_number(i, chain.standard_derivation_paths_for_account_number(i))).collect()
    }

    pub fn verify_and_assign_added_derivation_paths<DP: DerivationPath<P>>(&mut self, derivation_paths: Vec<DP>) {
        derivation_paths.iter().enumerate().for_each(|(&i, &path)| {
            match path.get_reference() {
                Reference::BIP32 => {
                    if self.bip32derivation_path.is_some() {
                        assert!(true, "There should only be one BIP 32 derivation path");
                    }
                    self.bip32derivation_path = Some(path as FundsPath<P>);
                },
                Reference::BIP44 => {
                    if self.bip44derivation_path.is_some() {
                        assert!(true, "There should only be one BIP 44 derivation path");
                    }
                    self.bip44derivation_path = Some(path as FundsPath<P>);
                },
                Reference::ContactBasedFundsRoot => {
                    if self.master_contacts_derivation_path.is_some() {
                        assert!(true, "There should only be one master contacts derivation path");
                    }
                    self.master_contacts_derivation_path = Some(path);
                },
                _ => {}
            }
            (i+1..derivation_paths.len()).into_iter().for_each(|path2| {
                assert_ne!(path, path2, "Derivation paths should all be different");
            });
        });
    }

    - (instancetype)initWithAccountNumber:(uint32_t)accountNumber withDerivationPaths:(NSArray<DSFundsDerivationPath *> *)derivationPaths inContext:(NSManagedObjectContext *)context {
    NSParameterAssert(derivationPaths);

    if (!(self = [super init])) return nil;
    _accountNumber = accountNumber;
    [self verifyAndAssignAddedDerivationPaths:derivationPaths];
    self.mFundDerivationPaths = [NSMutableArray array];
    self.mContactIncomingFundDerivationPathsDictionary = [NSMutableDictionary dictionary];
    self.mContactOutgoingFundDerivationPathsDictionary = [NSMutableDictionary dictionary];
    for (DSDerivationPath *derivationPath in derivationPaths) {
    if ([derivationPath isKindOfClass:[DSFundsDerivationPath class]]) {
    [self.mFundDerivationPaths addObject:(DSFundsDerivationPath *)derivationPath];
    }
    derivationPath.account = self;
    }
    self.transactions = [NSMutableOrderedSet orderedSet];
    self.allTx = [NSMutableDictionary dictionary];
    self.managedObjectContext = context ? context : [NSManagedObjectContext chainContext];
    self.transactionsToSave = [NSMutableArray array];
    self.transactionsToSaveInBlockSave = [NSMutableDictionary dictionary];
    self.isViewOnlyAccount = FALSE;
    return self;
}

}
