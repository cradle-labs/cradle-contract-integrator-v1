use crate::utils::functions::listing_factory::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters};
use num_bigint::BigUint;
use tokio::time::Duration;

pub async fn create_listing(
    args: CreateListing,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<String>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

    transaction.contract_id(contract_ids.cradle_listing_factory_contract_id);

    let mut params = ContractFunctionParameters::new();

    params.add_address(&args.fee_collector_address);
    params.add_address(&args.reserve_account);
    let max_supply = BigUint::from(args.max_supply);
    params.add_uint256(max_supply);
    params.add_address(&args.listing_asset);
    params.add_address(&args.purchase_asset);
    let purchase_price = BigUint::from(args.purchase_price);
    params.add_uint256(purchase_price);
    params.add_address(&args.beneficiary_address);
    params.add_address(&args.shadow_asset);

    transaction.function_with_parameters("createListing", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let tx_id = response.transaction_id.to_string();

    let record = response
        .get_record(&wallet.client)
        .await?
        .contract_function_result
        .ok_or_else(|| anyhow!("Failed to get transaction function return"))?;

    let listing_address = record
        .get_address(0)
        .ok_or_else(|| anyhow!("Failed to retrieve listing address"))?;

    let output = FunctionCallOutput {
        transaction_id: tx_id,
        output: Some(listing_address),
    };

    Ok(output)
}
