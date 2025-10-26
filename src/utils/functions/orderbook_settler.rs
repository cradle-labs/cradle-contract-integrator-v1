use hedera::{ContractExecuteTransaction, ContractFunctionParameters};
use num_bigint::BigUint;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use crate::utils::functions::commons::ContractFunctionProcessor;
use tokio::time::Duration;

pub struct SettleOrderInputArgs {
    pub bidder: String,
    pub asker: String,
    pub bid_asset: String,
    pub ask_asset: String,
    pub bid_asset_amount: u64,
    pub ask_asset_amount: u64
}

pub enum OrderBookSettlerFunctionInput {
    SettleOrder(SettleOrderInputArgs)
}


pub enum OrderBookSettlerFunctionOutput {
    SettleOrder(FunctionCallOutput<()>)
}

impl ContractFunctionProcessor<OrderBookSettlerFunctionOutput> for OrderBookSettlerFunctionInput {
    async fn process(&self, wallet: &mut ActionWallet) -> anyhow::Result<OrderBookSettlerFunctionOutput> {
        let contract_ids = wallet.get_contract_ids()?;
        let mut transaction = ContractExecuteTransaction::new();
        transaction.contract_id(contract_ids.orderbook_settler);

        let mut params = ContractFunctionParameters::new();

        match self {
            OrderBookSettlerFunctionInput::SettleOrder(args)=>{

                params.add_address(&args.bidder);
                params.add_address(&args.asker);
                params.add_address(&args.ask_asset);
                params.add_address(&args.bid_asset);

                let bid_asset_amount = BigUint::from(args.bid_asset_amount);
                let ask_asset_amount = BigUint::from(args.ask_asset_amount);

                params.add_uint256(bid_asset_amount);
                params.add_uint256(ask_asset_amount);

                transaction.function_with_parameters("settleOrder", &params);

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;

                let output = FunctionCallOutput {
                    transaction_id: response.transaction_id.to_string(),
                    output: None
                };

                Ok(OrderBookSettlerFunctionOutput::SettleOrder(output))
            }
        }
    }
}