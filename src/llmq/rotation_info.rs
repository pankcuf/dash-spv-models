use crate::llmq::{LLMQSnapshot, MNListDiff};
use crate::masternode::LLMQEntry;

#[derive(Debug)]
pub struct LLMQRotationInfo<'a> {
    pub snapshot_at_h_c: LLMQSnapshot,
    pub snapshot_at_h_2c: LLMQSnapshot,
    pub snapshot_at_h_3c: LLMQSnapshot,
    pub snapshot_at_h_4c: Option<LLMQSnapshot>, // exist only if extra_share is true
    pub mn_list_diff_tip: MNListDiff<'a>,
    pub mn_list_diff_at_h: MNListDiff<'a>,
    pub mn_list_diff_at_h_c: MNListDiff<'a>,
    pub mn_list_diff_at_h_2c: MNListDiff<'a>,
    pub mn_list_diff_at_h_3c: MNListDiff<'a>,
    pub mn_list_diff_at_h_4c: Option<MNListDiff<'a>>, // exist only if extra_share is true
    pub extra_share: bool,
    pub last_quorum_per_index: Vec<LLMQEntry>,
    pub quorum_snapshot_list: Vec<LLMQSnapshot>,
    pub mn_list_diff_list: Vec<MNListDiff<'a>>,
}
