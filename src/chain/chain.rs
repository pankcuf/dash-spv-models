use crate::common::ChainType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Chain {
    pub r#type: ChainType,
}
