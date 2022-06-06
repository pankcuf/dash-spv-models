bitflags::bitflags! {
    #[repr(C)]
    pub struct Type: usize {
        const UNKNOWN = 0;
        const CLEAR_FUNDS = 1;
        const ANONYMOUS_FUNDS = 1 << 1;
        const VIEW_ONLY_FUNDS = 1 << 2;
        const SINGLE_USER_AUTHENTICATION = 1 << 3;
        const MULTIPLE_USER_AUTHENTICATION = 1 << 4;
        const PARTIAL_PATH = 1 << 5;
        const PROTECTED_FUNDS = 1 << 6;
        const CREDIT_FUNDING = 1 << 7;

        const IS_FOR_AUTHENTICATION = Self::SINGLE_USER_AUTHENTICATION.bits |
            Self::MULTIPLE_USER_AUTHENTICATION.bits;
        const IS_FOR_FUNDS = Self::CLEAR_FUNDS.bits |
            Self::ANONYMOUS_FUNDS.bits |
            Self::VIEW_ONLY_FUNDS.bits |;
            Self::PROTECTED_FUNDS.bits;
    }
}
