
#[derive(PartialEq, Eq, Debug)]
pub enum Reference {
    Unknown = 0,
    BIP32 = 1,
    BIP44 = 2,
    Identities = 3,
    ProviderFunds = 4,
    ProviderVotingKeys = 5,
    ProviderOperatorKeys = 6,
    ProviderOwnerKeys = 7,
    ContactBasedFunds = 8,
    ContactBasedFundsRoot = 9,
    ContactBasedFundsExternal = 10,
    IdentityCreditRegistrationFunding = 11,
    IdentityCreditTopUpFunding = 12,
    IdentityCreditInvitationFunding = 13,
    Root = 255,
}

impl Reference {
    /// the reference of type of derivation path
    pub fn name() -> &str {
        match Self {
            Reference::Unknown => "Unknown",
            Reference::BIP32 => "BIP 32",
            Reference::BIP44 => "BIP 44",
            Reference::Identities => "Blockchain Identities",
            Reference::ProviderFunds => "Provider Holding Funds Keys",
            Reference::ProviderVotingKeys => "Provider Voting Keys",
            Reference::ProviderOperatorKeys => "Provider Operator Keys",
            Reference::ProviderOwnerKeys => "Provider Owner Keys",
            Reference::ContactBasedFunds => "Contact Funds",
            Reference::ContactBasedFundsRoot => "Contact Funds Root",
            Reference::ContactBasedFundsExternal => "Contact Funds External",
            Reference::IdentityCreditRegistrationFunding => "BI Credit Registration Funding",
            Reference::IdentityCreditTopUpFunding => "BI Credit Topup Funding",
            Reference::IdentityCreditInvitationFunding => "BI Credit Invitation Funding",
            Reference::Root => "Root",
        }
    }
}
