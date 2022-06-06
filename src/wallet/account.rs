use crate::tx::{CoinbaseTransaction, Transaction, UTXO};
use crate::tx::transaction::ITransaction;
use crate::wallet::wallet::Wallet;

pub struct Account<'a> {
    //@property (nonatomic, readonly) NSManagedObjectContext *managedObjectContext;

    // BIP 43 derivation paths
    pub fund_derivation_paths: Option<Vec<DerivationPath>>,
    pub outgoing_fund_derivation_paths: Option<Vec<DerivationPath>>,
    pub default_derivation_path: Option<DerivationPath>,
    pub bip44derivation_path: Option<DerivationPath>,
    pub bip32derivation_path: Option<DerivationPath>,
    pub master_contacts_derivation_path: Option<DerivationPath>,

    pub wallet: Option<Wallet>, //weak

    pub unique_id: &'a str,
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
    pub coinbase_transactions: Vec<CoinbaseTransaction<'a>>,

    /// Does this account have any coinbase rewards
    pub has_coinbase_transaction: bool,

    /// returns the first unused external address
    pub receive_address: Option<&'a str>,

    /// returns the first unused internal address
    pub change_address: Option<&'a str>,

    /// all previously generated external addresses
    pub external_addresses: Vec<&'a str>,

    /// all previously generated internal addresses
    pub internal_addresses: Vec<&'a str>,

    /// all the contacts for an account
    pub contacts: Vec<PotentialOneWayFriendship>,

    /// has an extended public key missing in one of the account derivation paths
    pub has_an_extended_public_key_missing: bool,
}

impl<'a> Account<'a> {
    pub fn verify_and_assign_added_derivation_paths(derivation_paths: Vec<DerivationPath>) {
        for derivation_path in derivation_paths {
            
        }
    }
    - (void)verifyAndAssignAddedDerivationPaths:(NSArray<DSDerivationPath *> *)derivationPaths {
    for (int i = 0; i < [derivationPaths count]; i++) {
    DSDerivationPath *derivationPath = [derivationPaths objectAtIndex:i];
    if (derivationPath.reference == DSDerivationPathReference_BIP32) {
    if (self.bip32DerivationPath) {
    NSAssert(TRUE, @"There should only be one BIP 32 derivation path");
    }
    self.bip32DerivationPath = (DSFundsDerivationPath *)derivationPath;
    } else if (derivationPath.reference == DSDerivationPathReference_BIP44) {
    if (self.bip44DerivationPath) {
    NSAssert(TRUE, @"There should only be one BIP 44 derivation path");
    }
    self.bip44DerivationPath = (DSFundsDerivationPath *)derivationPath;
    } else if (derivationPath.reference == DSDerivationPathReference_ContactBasedFundsRoot) {
    if (self.masterContactsDerivationPath) {
    NSAssert(TRUE, @"There should only be one master contacts derivation path");
    }
    self.masterContactsDerivationPath = derivationPath;
    }
    for (int j = i + 1; j < [derivationPaths count]; j++) {
    DSDerivationPath *derivationPath2 = [derivationPaths objectAtIndex:j];
    NSAssert([derivationPath isDerivationPathEqual:derivationPath2] == NO, @"Derivation paths should all be different");
    }
    //to do redo this check
    //        if ([self.mFundDerivationPaths count] || i != 0) {
    //            NSAssert(([derivationPath indexAtPosition:[derivationPath length] - 1] & ~(BIP32_HARD)) == _accountNumber, @"all derivationPaths need to be on same account");
    //        }
    }
    }


    pub fn init_with_account_number(
        account_number: u32,
        derivation_paths: Vec<FundsDerivationPath>
    ) -> Account<'a> {
        Account {
            account_number,

        }
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
