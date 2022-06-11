#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChainType {
    MainNet = 0,
    TestNet = 1,
    DevNet = 2,
}

impl ChainType {
    pub fn coin_type(&self) -> u64 {
        match Self {
            ChainType::MainNet => 5,
            _ => 1
        }
    }
}
