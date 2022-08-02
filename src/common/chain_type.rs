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
}
