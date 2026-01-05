use crate::utils::functions::cradle_native_listing::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use num_bigint::BigUint;
use tokio::time::Duration;

pub async fn update_listing_status(
    contract_id: String,
    status: ListingStatus,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(contract_id.parse()?);
    params.add_uint8(status.to_u8());
    transaction.function_with_parameters("updateListingStatus", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let output = FunctionCallOutput {
        transaction_id: response.transaction_id.to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn purchase(
    contract_id: String,
    args: PurchaseInputArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<u64>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(contract_id.parse()?);

    params.add_address(&args.buyer);
    let valid_amount = BigUint::from(args.amount);
    params.add_uint256(valid_amount);

    transaction.function_with_parameters("purchase", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let record = response
        .get_record(&wallet.client)
        .await?
        .contract_function_result
        .ok_or_else(|| anyhow!("Unable to retrieve result"))?;

    let received_listing_assets: u64 = record
        .get_u256(0)
        .ok_or_else(|| anyhow!("Unable to receive listing"))?
        .try_into()?;

    let output = FunctionCallOutput {
        transaction_id: response.transaction_id.to_string(),
        output: Some(received_listing_assets),
    };

    Ok(output)
}

pub async fn return_asset(
    contract_id: String,
    args: ReturnAssetInputArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<u64>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(contract_id.parse()?);

    params.add_address(&args.account);
    let valid_amount = BigUint::from(args.amount);
    params.add_uint256(valid_amount);

    transaction.function_with_parameters("returnAsset", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let record = response
        .get_record(&wallet.client)
        .await?
        .contract_function_result
        .ok_or_else(|| anyhow!("Unable to retrieve result"))?;

    let received_purchase_asset: u64 = record
        .get_u256(0)
        .ok_or_else(|| anyhow!("Unable to extract received amount"))?
        .try_into()?;

    let output = FunctionCallOutput {
        transaction_id: response.transaction_id.to_string(),
        output: Some(received_purchase_asset),
    };

    Ok(output)
}

pub async fn withdraw_to_beneficiary(
    contract_id: String,
    args: WithdrawToBeneficiaryInputArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(contract_id.parse()?);

    let valid_amount = BigUint::from(args.amount);
    params.add_uint256(valid_amount);

    transaction.function_with_parameters("withdrawToBeneficiary", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let output = FunctionCallOutput {
        transaction_id: response.transaction_id.to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn get_listing_stats(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<ListingStats>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(10_000_000);
    query_transaction.contract_id(contract_id.parse()?);
    query_transaction.function("getListingStats");

    let response = query_transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let total_distributed: u64 = response
        .get_u256(0)
        .ok_or_else(|| anyhow!("Unable to extract result"))?
        .try_into()?;
    let remaining: u64 = response
        .get_u256(1)
        .ok_or_else(|| anyhow!("Unable to extract result"))?
        .try_into()?;
    let raised: u64 = response
        .get_u256(2)
        .ok_or_else(|| anyhow!("Unable to extract result"))?
        .try_into()?;
    let balance: u64 = response
        .get_u256(3)
        .ok_or_else(|| anyhow!("Unable to extract result"))?
        .try_into()?;
    let status: u8 = response
        .get_u8(4)
        .ok_or_else(|| anyhow!("unable to extract result"))?;

    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(ListingStats {
            total_distributed,
            remaining,
            raised,
            balance,
            status: ListingStatus::from(status),
        }),
    };

    Ok(output)
}

pub async fn get_fee(
    contract_id: String,
    amount: u64,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<u64>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    query_transaction.contract_id(contract_id.parse()?);

    let amount_bn = BigUint::from(amount);
    params.add_uint256(amount_bn);

    query_transaction.function_with_parameters("getFee", &params);

    let response = query_transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let fee_applied: u64 = response
        .get_u256(0)
        .ok_or_else(|| anyhow!("unable to extract fee"))?
        .try_into()?;

    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(fee_applied),
    };

    Ok(output)
}
