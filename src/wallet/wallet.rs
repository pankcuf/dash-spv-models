use std::collections::HashMap;
use crate::wallet::account::Account;


#[derive(Clone)]
pub struct Wallet<'a> {
    pub ordered_accounts: HashMap<u64, Account<'a>>,
    pub last_account_number: u32,
    pub accounts: Vec<Account<'a>>,
    pub specialTransactionsHolder

    @property (nonatomic, readonly) DSSpecialTransactionsWalletHolder *specialTransactionsHolder;

    @property (nonatomic, readonly) NSDictionary<NSData *, DSBlockchainIdentity *> *blockchainIdentities;

    @property (nonatomic, readonly) NSDictionary<NSData *, DSBlockchainInvitation *> *blockchainInvitations;

    @property (nonatomic, readonly, nullable) DSBlockchainIdentity *defaultBlockchainIdentity;

    - (void)setDefaultBlockchainIdentity:(DSBlockchainIdentity *)defaultBlockchainIdentity;

    @property (nonatomic, readonly) NSArray<NSString *> *blockchainIdentityAddresses;

    @property (nonatomic, readonly) NSArray<NSString *> *providerOwnerAddresses;

    @property (nonatomic, readonly) NSArray<NSString *> *providerVotingAddresses;

    @property (nonatomic, readonly) NSArray<NSString *> *providerOperatorAddresses;

//This is unique among all wallets and all chains
    @property (nonatomic, readonly) NSString *uniqueIDString;

    @property (nonatomic, readonly) NSTimeInterval walletCreationTime;

// set to true if this wallet is not stored on disk
    @property (nonatomic, readonly, getter=isTransient) BOOL transient;

// chain for the wallet
    @property (nonatomic, readonly) DSChain *chain;

// current wallet balance excluding transactions known to be invalid
    @property (nonatomic, readonly) uint64_t balance;

// all previously generated external addresses
    @property (nonatomic, readonly) NSSet<NSString *> *allReceiveAddresses;

// all previously generated internal addresses
    @property (nonatomic, readonly) NSSet<NSString *> *allChangeAddresses;

// NSValue objects containing UTXO structs
    @property (nonatomic, readonly) NSArray *unspentOutputs;

// latest 100 transactions sorted by date, most recent first
    @property (nonatomic, readonly) NSArray<DSTransaction *> *recentTransactions;

// all wallet transactions sorted by date, most recent first
    @property (nonatomic, readonly) NSArray<DSTransaction *> *allTransactions;

// the total amount spent from the wallet (excluding change)
    @property (nonatomic, readonly) uint64_t totalSent;

// the total amount received by the wallet (excluding change)
    @property (nonatomic, readonly) uint64_t totalReceived;

// the first unused index for blockchain identity registration funding
    @property (nonatomic, readonly) uint32_t unusedBlockchainIdentityIndex;

// the first unused index for invitations
    @property (nonatomic, readonly) uint32_t unusedBlockchainInvitationIndex;

// the amount of known blockchain identities
    @property (nonatomic, readonly) uint32_t blockchainIdentitiesCount;

// the amount of known blockchain invitations
    @property (nonatomic, readonly) uint32_t blockchainInvitationsCount;

// The fingerprint for currentTransactions
    @property (nonatomic, readonly) NSData *chainSynchronizationFingerprint;

}
