use dash_spv_primitives::crypto::byte_util::Reversable;
use dash_spv_primitives::crypto::UInt256;
use dash_spv_primitives::hashes::hex::FromHex;
use crate::common::ChainType::{DevNet, MainNet, TestNet};
use crate::tx::transaction::TX_FEE_PER_B;
use crate::wallet::wallet::DUFFS;

// highest value for difficulty target (higher values are less difficult)
pub const MAX_PROOF_OF_WORK_MAINNET: UInt256 = UInt256::from_hex("00000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap().reversed();
pub const MAX_PROOF_OF_WORK_TESTNET: UInt256 = UInt256::from_hex("00000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap().reversed();
pub const MAX_PROOF_OF_WORK_DEVNET: UInt256 = UInt256::from_hex("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap().reversed();

pub const SPORK_PUBLIC_KEY_MAINNET: &str = "04549ac134f694c0243f503e8c8a9a986f5de6610049c40b07816809b0d1d06a21b07be27b9bb555931773f62ba6cf35a25fd52f694d4e1106ccd237a7bb899fdd";
pub const SPORK_PUBLIC_KEY_TESTNET: &str = "046f78dcf911fbd61910136f7f0f8d90578f68d0b3ac973b5040fb7afb501b5939f39b108b0569dca71488f5bbf498d92e4d1194f6f941307ffd95f75e76869f0e";


pub const SPORK_ADDRESS_MAINNET: &str = "Xgtyuk76vhuFW2iT7UAiHgNdWXCf3J34wh";
pub const SPORK_ADDRESS_TESTNET: &str = "yjPtiKh2uwk3bDutTEA2q9mCtXyiZRWn55";

pub const MAINNET_DASHPAY_CONTRACT_ID: &str = "";
pub const MAINNET_DPNS_CONTRACT_ID: &str = "";

pub const TESTNET_DASHPAY_CONTRACT_ID: &str = "2Vuou3EfbrtunwCZvQp1XS5PXZ5CgC1pGBz4VPT4ojmy";
pub const TESTNET_DPNS_CONTRACT_ID: &str = "Bw9PUC3aSEGQ4j5qrvpNLrRNFPVMiUHZLr1atgfYJcmf";

pub const DASH_PUBKEY_ADDRESS: u8 = 76;
pub const DASH_SCRIPT_ADDRESS: u8 = 16;
pub const DASH_PUBKEY_ADDRESS_TEST: u8 = 140;
pub const DASH_SCRIPT_ADDRESS_TEST: u8 = 19;
pub const DASH_PRIVKEY: u8 = 204;
pub const DASH_PRIVKEY_TEST: u8 = 239;


pub const DEFAULT_FEE_PER_B: u64 = TX_FEE_PER_B;
// minimum relay fee on a 191byte tx
pub const MIN_FEE_PER_B: u64 = TX_FEE_PER_B;
// slightly higher than a 1000bit fee on a 191byte tx
pub const MAX_FEE_PER_B: u64 = 1000;

//This is about the time if we consider a block every 10 mins (for 500 blocks)
pub const WEEK_TIME_INTERVAL: u64 = 604800; //7*24*60*60

pub const HEADER_WINDOW_BUFFER_TIME: u64 = (WEEK_TIME_INTERVAL / 2);


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChainType {
    MainNet,
    TestNet,
    DevNet(Option<String>),
}

impl ChainType {
    pub fn coin_type(&self) -> u64 {
        match self {
            MainNet => 5,
            _ => 1
        }
    }

    /// The network name. Currently main, test, dev or reg.
    pub fn network_name(&self) -> &str {
        match self {
            MainNet => "main",
            TestNet => "test",
            DevNet(name) => name | "dev"
        }
    }
    /*! @brief The name of the chain (Mainnet-Testnet-Devnet).  */
    pub fn name(&self) -> &str {
        match self {
            MainNet => "Mainnet",
            TestNet => "Testnet",
            DevNet(name) => format!("Devnet - %@.%u")
            //     if (_networkName) return _networkName;
//     return [NSString stringWithFormat:@"Devnet - %@.%u", self.devnetIdentifier, self.devnetVersion];
        }
    }

    pub fn transaction_version(&self) -> u16 {
        match self {
            DevNet(_) => 3,
            _ => 1
        }
    }

    pub fn peer_misbehaving_threshold(&self) -> u16 {
        match self {
            MainNet => 20,
            TestNet => 40,
            DevNet(_) => 3
        }
    }

    pub fn magic_number(&self) -> u32 {
        match self {
            MainNet => 0xbd6b0cbf,
            TestNet => 0xffcae2ce,
            DevNet(_) => 0xceffcae2,
        }
    }

    pub fn default_standard_port(&self) -> u16 {
        match self {
            MainNet => 9999,
            TestNet => 19999,
            DevNet(_) => 20001,
        }
    }

    pub fn default_headers_max_amount(&self) -> u16 {
        match self {
            MainNet => 2000,
            TestNet => 2000,
            DevNet(_) => 2000,
        }
    }

    pub fn dapi_grpc_standard_port(&self) -> u16 {
        match self {
            MainNet => 3010,
            TestNet => 3010,
            DevNet(_) => 3010,
        }
    }

    pub fn dapi_jrpc_standard_port(&self) -> u16 {
        match self {
            MainNet => 3000,
            TestNet => 3000,
            DevNet(_) => 3000,
        }
    }

    pub fn protocol_version(&self) -> u16 {
        match self {
            MainNet => 70219,
            TestNet => 70220,
            DevNet(_) => 70221,
        }
    }

    pub fn default_min_protocol_version(&self) -> u16 {
        match self {
            MainNet => 70218,
            TestNet => 70218,
            DevNet(_) => 70219,
        }
    }

    pub fn platform_protocol_version(&self) -> u16 {
        match self {
            MainNet => 1,
            TestNet => 1,
            DevNet(_) => 1,
        }
    }

    pub fn default_min_platform_protocol_version(&self) -> u16 {
        match self {
            MainNet => 1,
            TestNet => 1,
            DevNet(_) => 1,
        }
    }

    pub fn max_target_proof_of_work(&self) -> u32 {
        match self {
            MainNet => 0x1e0fffff, // highest value for difficulty target (higher values are less difficult)
            TestNet => 0x1e0fffff,
            DevNet(_) => 0x207fffff,
        }
    }

    pub fn max_proof_of_work(&self) -> UInt256 {
        match self {
            MainNet => MAX_PROOF_OF_WORK_MAINNET,
            TestNet => MAX_PROOF_OF_WORK_TESTNET,
            DevNet(_) => MAX_PROOF_OF_WORK_DEVNET,
        }
    }

    pub fn base_reward(&self) -> u64 {
        match self {
            MainNet => 5 * DUFFS,
            _ => 50 * DUFFS,
        }
    }

    pub fn allow_min_difficulty_blocks(&self) -> bool {
        self != MainNet
    }

    pub fn public_key_address(&self) -> u8 {
        if self == MainNet {
            DASH_PUBKEY_ADDRESS
        } else {
            DASH_PUBKEY_ADDRESS_TEST
        }
    }

}

