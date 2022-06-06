pub mod common;
pub mod llmq;
pub mod masternode;
pub mod tx;
pub mod identity;
pub mod wallet;
pub mod derivation_paths;
pub mod keys;
pub mod chain;

extern crate bitflags;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
