use dash_spv_primitives::crypto::UInt256;

pub struct TransientDashpayUser<'a> {
    pub avatar_fingerprint: Option<i64>,
    pub avatar_hash: Option<UInt256>,
    pub avatar_path: Option<&'a str>,
    pub display_name: Option<&'a str>,
    pub public_message: Option<&'a str>,
    pub revision: i32,
    pub document_id: Option<UInt256>,
    pub created_at: u64, //NSTimeInterval
    pub updated_at: u64, //NSTimeInterval
}
