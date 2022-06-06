bitflags::bitflags! {
    #[repr(C)]
    pub struct RegistrationStep: usize {
        const NONE = 0;
        const FUNDING_TRANSACTION_CREATION = 1;
        const FUNDING_TRANSACTION_ACCEPTED = 2;
        const LOCAL_IN_WALLET_PERSISTENCE = 4;
        const PROOF_AVAILABLE = 8;
        const L1_STEPS = Self::FUNDING_TRANSACTION_CREATION.bits |
            Self::FUNDING_TRANSACTION_ACCEPTED.bits |
            Self::LOCAL_IN_WALLET_PERSISTENCE.bits |
            Self::LOCAL_IN_WALLET_PERSISTENCE.bits;
        const IDENTITY = 16;
        const REGISTRATION_STEPS = Self::L1_STEPS.bits | Self::IDENTITY.bits;
        const USERNAME = 32;
        const REGISTRATION_STEPS_WITH_USERNAME = Self::REGISTRATION_STEPS.bits | Self::USERNAME.bits;
        const PROFILE = 64;
        const ALL = Self::REGISTRATION_STEPS_WITH_USERNAME.bits | Self::PROFILE.bits;
        const CANCELLED = 1 << 30;
    }
}
