use crate::tx::transaction::ITransaction;
use crate::wallet::wallet::Wallet;

pub struct SpecialTransactionWalletHolder {
    pub all_transactions: Vec<dyn ITransaction>,


    - (instancetype)initWithWallet:(DSWallet *)wallet inContext:(NSManagedObjectContext *_Nullable)managedObjectContext;

    - (DSTransaction *_Nullable)transactionForHash:(UInt256)transactionHash;

    - (BOOL)registerTransaction:(DSTransaction *)transaction saveImmediately:(BOOL)saveImmediately;

    - (void)removeAllTransactions;

    - (DSCreditFundingTransaction *)creditFundingTransactionForBlockchainIdentityUniqueId:(UInt256)blockchainIdentityUniqueId;

//// This gets a blockchain user registration transaction that has a specific public key hash (will change to BLS pub key)
//- (DSBlockchainIdentityRegistrationTransition*)blockchainIdentityRegistrationTransactionForPublicKeyHash:(UInt160)publicKeyHash;
//
//// This gets a blockchain user reset transaction that has a specific public key hash (will change to BLS pub key)
//- (DSBlockchainIdentityUpdateTransition*)blockchainIdentityResetTransactionForPublicKeyHash:(UInt160)publicKeyHash;
//
//- (NSArray<DSTransaction*>*)identityTransitionsForRegistrationTransitionHash:(UInt256)blockchainIdentityRegistrationTransactionHash;
//
//- (UInt256)lastSubscriptionTransactionHashForRegistrationTransactionHash:(UInt256)blockchainIdentityRegistrationTransactionHash;

// this is used to save transactions atomically with the block, needs to be called before switching threads to save the block
    - (void)prepareForIncomingTransactionPersistenceForBlockSaveWithNumber:(uint32_t)blockNumber;

// this is used to save transactions atomically with the block
    - (void)persistIncomingTransactionsAttributesForBlockSaveWithNumber:(uint32_t)blockNumber inContext:(NSManagedObjectContext *)context;

    - (NSArray *)setBlockHeight:(int32_t)height andTimestamp:(NSTimeInterval)timestamp forTransactionHashes:(NSArray *)txHashes;

}

impl SpecialTransactionWalletHolder {
    pub fn init_with_wallet(wallet: Wallet)
}
