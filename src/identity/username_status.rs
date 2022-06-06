#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum UsernameStatus {
    NotPresent = 0,
    Initial = 1,
    PreorderRegistrationPending = 2,
    Preordered = 3,
    RegistrationPending = 4, //sent to DAPI, not yet confirmed
    Confirmed = 5,
    TakenOnNetwork = 6,
}
