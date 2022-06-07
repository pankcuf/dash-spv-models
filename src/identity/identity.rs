use dash_spv_primitives::crypto::byte_util::AsBytes;
use dash_spv_primitives::crypto::UInt256;
use dash_spv_primitives::util::base58::encode_slice;
use crate::identity::invitation::Invitation;
use crate::identity::registration_status::RegistrationStatus;
use crate::identity::registration_step::RegistrationStep;
use crate::identity::transient_dashpay_user::TransientDashpayUser;
use crate::tx::{CreditFundingTransaction, UTXO};
use crate::wallet::wallet::Wallet;

pub struct Identity<'a> {

    /// This is the unique identifier representing the blockchain identity.
    /// It is derived from the credit funding transaction credit burn UTXO (as of dpp v10).
    /// Returned as a 256 bit number.
    pub unique_id: UInt256,

    /// This is the outpoint of the registration credit funding transaction.
    /// It is used to determine the unique ID by double SHA256 its value.
    /// Returned as a UTXO { .hash , .n }
    pub locked_outpoint: UTXO,

    /// This is if the blockchain identity is present in wallets or not.
    /// If this is false then the identity is known for example from being a dashpay friend.
    pub is_local: bool,

    /// This is if the blockchain identity is made for being an invitation.
    /// All invitations should be marked as non local as well.
    pub is_outgoing_invitation: bool,

    /// This is if the blockchain identity is made from an invitation we received.
    pub is_from_incoming_invitation: bool,

    /// This is TRUE if the blockchain identity is an effemeral identity returned when searching.
    pub is_transient: bool,

    /// This references transient Dashpay user info if on a transient blockchain identity.
    pub transient_dashpay_user: TransientDashpayUser<'a>,

    /// This is the bitwise steps that the identity has already performed in registration.
    pub steps_completed: RegistrationStep,

    /// This is the wallet holding the blockchain identity.
    /// There should always be a wallet associated to an identity if the identity is local,
    /// but never if it is not.
    pub wallet: Option<Wallet>,

    /// This is invitation that is identity originated from.
    pub associated_invitation: Invitation,

    /// This is the index of the blockchain identity in the wallet.
    /// The index is the top derivation used to derive an extended set of keys for the identity.
    /// No two local blockchain identities should be allowed to have the same index in a wallet.
    /// For example m/.../.../.../index/key
    pub index: u32,

    /// Related to DPNS.
    /// This is the list of usernames that are associated to the identity in the domain "dash".
    /// These usernames however might not yet be registered or might be invalid.
    /// This can be used in tandem with the statusOfUsername: method
    pub dashpay_usernames: Vec<&'a str>,

    /// Related to DPNS.
    /// This is the list of usernames with their .dash domain that are associated to the identity
    /// in the domain "dash".
    /// These usernames however might not yet be registered or might be invalid.
    /// This can be used in tandem with the statusOfUsername: method
    pub dashpay_username_full_paths: Vec<&'a str>,

    /// Related to DPNS.
    /// This is current and most likely username associated to the identity.
    /// It is not necessarily registered yet on L2 however so its state should be determined with
    /// the statusOfUsername: method
    /// There are situations where this is nil as it is not yet known or if no username has yet been set
    pub current_dashpay_username: Option<&'a str>,

    /// Related to registering the identity.
    /// This is the address used to fund the registration of the identity.
    /// Dash sent to this address in the special credit funding transaction will be converted to
    /// L2 credits
    pub registration_funding_address: &'a str,

    /// The known balance in credits of the identity
    pub credit_balance: u64,

    /// The number of registered active keys that the blockchain identity has
    pub active_key_count: u32,

    /// The number of all keys that the identity has, registered, in registration, or inactive
    pub total_key_count: u32,

    /// This is the transaction on L1 that has an output that is used to fund the creation of
    /// this identity. There are situations where this is nil as it is not yet known ;
    /// if the blockchain identity is being retrieved from L2 or if we are resyncing the chain.
    pub registration_credit_funding_transaction: Option<CreditFundingTransaction>,

    /// This is the hash of the transaction on L1 that has an output that is used to fund the
    /// creation of this identity. There are situations where this is nil as it is not yet known;
    /// if the blockchain identity is being retrieved from L2 or if we are resyncing the chain.
    pub registration_credit_funding_transaction_hash: Option<UInt256>,

    /// In our system a contact is a vue on a blockchain identity for Dashpay.
    /// A blockchain identity is therefore represented by a contact that will have relationships
    /// in the system. This is in the default backgroundContext.
    pub matchingDashpayUserInViewContext: DSDashpayUserEntity,

    /// This is the status of the registration of the identity. It starts off in an initial status,
    /// and ends in a confirmed status
    pub registration_status: RegistrationStatus,

    /// This is the localized status of the registration of the identity returned as a string.
    /// It starts off in an initial status, and ends in a confirmed status
    pub localized_registration_status_string: &'a str,

    /// This is a convenience method that checks to see if registrationStatus is confirmed
    pub is_registered: bool,

    /// This is a convenience factory to quickly make dashpay documents
    pub dashpay_document_factory: DocumentFactory,

    /// This is a convenience factory to quickly make dpns documents
    pub dpns_document_factory: DocumentFactory,

    /// Represents the last L1 block height for which Dashpay would be synchronized,
    /// if this isn't at the end of the chain then we need to query L2 to make sure
    /// we don't need to update our bloom filter
    pub dashpay_syncronization_block_height: u32,

    /// Represents the last L1 block hash for which Dashpay would be synchronized
    pub dashpay_syncronization_block_hash: UInt256,
}

impl Identity {

    /// This is the unique identifier representing the blockchain identity.
    /// It is derived from the credit funding transaction credit burn UTXO (as of dpp v10).
    /// Returned as a base 58 string of a 256 bit number.
    pub fn unique_id_string(&self) -> String {
        encode_slice(self.unique_id.as_bytes())
    }

    /// This is TRUE only if the blockchain identity is contained within a wallet.
    /// It could be in a cleanup phase where it was removed from the wallet but still being help
    /// in memory by callbacks.
    pub fn is_active(&self) -> bool {
        if self.is_local {
            match self.wallet {
                Some(wallet) => wallet.blockchainIdentities[self.uniqueIDData] != nil,
                None => false
            }
        } else {
            return self.ch
            return [self.chain.chainManager.identitiesManager foreignBlockchainIdentityWithUniqueId:self.uniqueID] != nil;
        }
    }
}
