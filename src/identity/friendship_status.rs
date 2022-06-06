bitflags::bitflags! {
    #[repr(C)]
    pub struct FriendshipStatus: usize {
        const UNKNOWN = usize::MAX;
        const NONE = 0;
        const OUTGOING = 1;
        const INCOMING = 2;
        const FRIENDS = Self::OUTGOING.bits | Self::INCOMING.bits;
    }
}
