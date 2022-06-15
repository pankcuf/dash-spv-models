use dash_spv_primitives::crypto::data_ops::uint256_from_long;
use crate::chain::chain::Chain;
use crate::chain::chain_parameters::ChainParameters;
use crate::{derivation_paths, keys};
use crate::derivation_paths::path::{DerivationPath, FEATURE_PURPOSE};
use crate::derivation_paths::Reference;
use crate::derivation_paths::simple_indexed_path::SimpleIndexedPath;
use crate::tx::transaction::ITransaction;

pub struct MasternodeHoldingsPath<P: ChainParameters> {
    pub base: SimpleIndexedPath<P>,
}

impl<P> MasternodeHoldingsPath<P> {

    pub fn provider_funds_derivation_path_for_chain(chain: Chain<P>) -> Self<P> {
        Self {
            base: SimpleIndexedPath::init_with_indexes(
                vec![
                    uint256_from_long(FEATURE_PURPOSE),
                    uint256_from_long(chain.chain_type.coin_type()),
                    uint256_from_long(3),
                    uint256_from_long(0),
                ],
                vec![true, true, true, true],
                derivation_paths::r#type::Type::ProtectedFunds,
                keys::Type::ECDSA,
                Reference::ProviderFunds,
                chain
            )
        }
    }

    pub fn receive_address(&mut self) -> Option<&String> {
        if let Some(addr) = self.base.register_addresses_with_gap_limit(1)?.last() {
            Some(addr)
        } else {
            self.base.ordered_addresses.last()
        }
    }

    fn default_gap_limit(&self) -> usize {
        5
    }

    // sign any inputs in the given transaction that can be signed using private keys from the wallet
    pub fn sign_transaction<TX: ITransaction, C: Fn(bool, bool)>(&self, transaction: TX, auth_prompt: String, completion: C) {

        let i_addrs = transaction.get_input_addresses();

        if i_addrs.len() != 1 {
            completion(false, false);
            return;
        }
        // if let Some(&addr) = i_addrs.first() {
        //     if let Some(index) = self.base.index_of_known_address(addr) {
        //         if let Some(wallet) = self.base.base.wallet {
        //             wallet.seedRequestBlock(authprompt, MASTERNODE_COST, ^void(NSData *_Nullable seed, BOOL cancelled) {
        //                 if (!seed) {
        //                     if (completion) completion(NO, cancelled);
        //                 } else {
        //                     DSECDSAKey *key = (DSECDSAKey *)[self privateKeyAtIndex:(uint32_t)index fromSeed:seed];
        //
        //                     BOOL signedSuccessfully = [transaction signWithPrivateKeys:@[key]];
        //                     if (completion) completion(signedSuccessfully, NO);
        //                 }
        //             });
        //         }
        //
        //
        //     }
        // }
    }
}

impl<P> DerivationPath<P> for MasternodeHoldingsPath<P> {

}
