use crate::utils::functions::orderbook_settler::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters};
use num_bigint::BigUint;
use tokio::time::Duration;

pub async fn settle_order(
    args: SettleOrderInputArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.cradle_order_book_settler_contract_id);
    transaction.gas(5_000_000);

    let mut params = ContractFunctionParameters::new();

    params.add_address(&args.bidder);
    params.add_address(&args.asker);
    params.add_address(&args.ask_asset);
    params.add_address(&args.bid_asset);

    let bid_asset_amount = BigUint::from(args.bid_asset_amount);
    let ask_asset_amount = BigUint::from(args.ask_asset_amount);

    params.add_uint256(bid_asset_amount);
    params.add_uint256(ask_asset_amount);

    transaction.function_with_parameters("settleOrder", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let output = FunctionCallOutput {
        transaction_id: response.transaction_id.to_string(),
        output: None,
    };

    Ok(output)
}
