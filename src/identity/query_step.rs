use crate::identity::registration_step::RegistrationStep;

bitflags::bitflags! {
    #[repr(C)]
    pub struct QueryStep: usize {
        const NONE = RegistrationStep::NONE.bits();
        const IDENTITY = RegistrationStep::IDENTITY.bits();
        const USERNAME = RegistrationStep::USERNAME.bits();
        const PROFILE = RegistrationStep::PROFILE.bits();
        const INCOMING_CONTACT_REQUESTS = 128;
        const OUTGOING_CONTACT_REQUESTS = 256;
        const CONTACT_REQUESTS = Self::INCOMING_CONTACT_REQUESTS.bits | Self::OUTGOING_CONTACT_REQUESTS.bits;
        const ALL_FOR_FOREIGN_BLOCKCHAIN_IDENTITY = Self::IDENTITY.bits | Self::USERNAME.bits | Self::PROFILE.bits;
        const ALL_FOR_LOCAL_BLOCKCHAIN_IDENTITY = Self::ALL_FOR_FOREIGN_BLOCKCHAIN_IDENTITY.bits | Self::CONTACT_REQUESTS.bits;
        const NO_IDENTITY = 1 << 28;
        const BAD_QUERY = 1 << 29;
        const CANCELLED = 1 << 30;
   }
}
