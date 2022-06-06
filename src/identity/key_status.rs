#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum KeyStatus {
    Unknown = 0,
    Registered = 1,
    Registering = 2,
    NotRegistered = 3,
    Revoked = 4,
}

