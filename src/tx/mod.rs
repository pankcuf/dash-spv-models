use byte::{BytesExt, LE};
use dash_spv_primitives::crypto::byte_util::BytesDecodable;
use dash_spv_primitives::impl_bytes_decodable_lt;

pub mod coinbase_transaction;
pub mod transaction;
pub mod utxo;
pub mod credit_funding_transaction;
pub mod direction;

pub use self::transaction::Transaction;
pub use self::transaction::TransactionInput;
pub use self::transaction::TransactionOutput;
pub use self::transaction::TransactionType;
pub use self::coinbase_transaction::CoinbaseTransaction;
pub use self::credit_funding_transaction::CreditFundingTransaction;
pub use self::utxo::UTXO;

impl_bytes_decodable_lt!(TransactionInput);
impl_bytes_decodable_lt!(TransactionOutput);
impl_bytes_decodable_lt!(Transaction);
impl_bytes_decodable_lt!(CoinbaseTransaction);
