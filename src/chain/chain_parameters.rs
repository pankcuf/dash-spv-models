use dash_spv_primitives::crypto::UInt256;
use crate::common::ChainType;

pub trait ChainParameters {
    fn r#type(&self) -> ChainType;
    fn cached_base_parameters(&self) -> BaseChainParameters;

    fn peer_misbehaving_threshold(&self) -> u32 {
        match self.r#type() {
            ChainType::MainNet => 20,
            ChainType::TestNet => 40,
            ChainType::DevNet(_) => 3,
        }
    }

}

pub struct BaseChainParameters {
    pub min_protocol_version: u32,
    pub protocol_version: u32,
    pub standard_port: u32,
    pub standard_dapi_jrpc_port: u32,
    pub standard_dapi_grpc_port: u32,
    pub max_proof_of_work: UInt256,
    pub dpns_contract_id: UInt256,
    pub dashpay_contract_id: UInt256,
}

pub struct MainnetParameters {

}

pub struct TestnetParameters {

}

pub struct DevnetParameters {
    /// The devnet identifier is the name of the devnet, the genesis hash of a devnet uses this
    /// devnet identifier in its construction.
    pub identifier: Option<String>,

    /// The devnet version is the version of the devnet, the genesis hash of a devnet uses this
    /// devnet identifier in its construction.
    pub version: u16,
}

impl ChainParameters for DevnetParameters {
    fn r#type(&self) -> ChainType {
        // ChainType::DevNet(_)
    }

    fn cached_base_parameters(&self) -> BaseChainParameters {
        todo!()
    }
}
