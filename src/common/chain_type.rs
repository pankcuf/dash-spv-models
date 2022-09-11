use dash_spv_primitives::crypto::byte_util::Reversable;
use dash_spv_primitives::crypto::UInt256;
use dash_spv_primitives::hashes::hex::FromHex;
use crate::common::LLMQType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChainType {
    MainNet = 0,
    TestNet = 1,
    DevNet = 2,
}

impl ChainType {
    pub fn is_mainnet(&self) -> bool {
        *self == ChainType::MainNet
    }

    pub fn genesis_hash(&self) -> UInt256 {
        match self {
            ChainType::MainNet => UInt256::from_hex(
                "00000ffd590b1485b3caadc19b22e6379c733355108f107a430458cdf3407ab6",
            ),
            ChainType::TestNet => UInt256::from_hex(
                "00000bafbc94add76cb75e2ec92894837288a481e5c005f6563d91623bf8bc2c",
            ),
            ChainType::DevNet => UInt256::from_hex(
                "00000bafbc94add76cb75e2ec92894837288a481e5c005f6563d91623bf8bc2c",
            ),
        }
        .unwrap()
        .reversed()
    }

    pub fn isd_llmq_type(&self) -> LLMQType {
        match self {
            ChainType::MainNet => LLMQType::Llmqtype60_75,
            ChainType::TestNet => LLMQType::Llmqtype60_75,
            ChainType::DevNet => LLMQType::LlmqtypeDevnetDIP0024,
        }
    }
}
