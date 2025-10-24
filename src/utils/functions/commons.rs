use crate::wallet::wallet::ActionWallet;
use anyhow::Result;
pub trait ContractFunctionProcessor<Output> {
    async fn process(&self, wallet: &mut ActionWallet)->Result<Output>;
}