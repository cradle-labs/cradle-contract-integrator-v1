use crate::utils::functions::asset_issuer::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, ContractId, Hbar};
use std::str::FromStr;
use tokio::time::Duration;

pub async fn create_asset(
    args: CreateAssetArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<CreateAssetResult>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

    let contract_id = ContractId::from_str(args.contract_id.as_str())?;
    transaction.contract_id(contract_id);
    transaction.function("createAsset");
    transaction.payable_amount(Hbar::new(20));

    let mut params = ContractFunctionParameters::new();
    params.add_string(args.name.as_str());
    params.add_string(args.symbol.as_str());
    params.add_address(args.acl_contract.as_str());
    params.add_uint64(args.allow_list);

    transaction.function_parameters(params.to_bytes(Some("createAsset")));

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let record = response.get_record(&mut wallet.client).await?;
    let result = record
        .contract_function_result
        .ok_or_else(|| anyhow!("Failed to find contract result"))?;
    let asset_manager = result
        .get_address(0)
        .ok_or_else(|| anyhow!("Failed to find asset manager"))?;
    let token_address = result
        .get_address(1)
        .ok_or_else(|| anyhow!("Failed to find token address"))?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: Some(CreateAssetResult {
            token: token_address,
            asset_manager,
        }),
    };

    Ok(output)
}

pub async fn lock_reserves(
    args: LockReservesArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

    let contract_id = ContractId::from_str(args.contract_id.as_str())?;
    transaction.contract_id(contract_id);

    let mut params = ContractFunctionParameters::new();
    let amount = num_bigint::BigUint::from(args.amount);
    params.add_address(args.user.as_str());
    params.add_uint256(amount);
    transaction.function_with_parameters("lockReserves", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn release_asset(
    args: ReleaseAssetArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

    let contract_id = ContractId::from_str(args.contract_id.as_str())?;
    transaction.contract_id(contract_id);
    transaction.function("releaseAsset");

    let mut params = ContractFunctionParameters::new();
    params.add_address(args.user.as_str());
    params.add_string(args.symbol.as_str());
    let mint_amount = num_bigint::BigUint::from(args.mint_amount);
    params.add_uint256(mint_amount);
    let unlock_amount = num_bigint::BigUint::from(args.unlock_amount);
    params.add_uint256(unlock_amount);
    transaction.function_parameters(params.to_bytes(Some("releaseAsset")));

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn lock_asset(
    args: LockAssetArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

    let contract_id = ContractId::from_str(args.contract_id.as_str())?;
    transaction.contract_id(contract_id);
    transaction.function("releaseAsset");

    let mut params = ContractFunctionParameters::new();
    params.add_address(args.user.as_str());
    params.add_address(args.asset.as_str());
    let amount = num_bigint::BigUint::from(args.amount);
    params.add_uint256(amount);
    transaction.function_parameters(params.to_bytes(Some("lockAsset")));

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };
    Ok(output)
}

pub async fn release_reserves(
    args: ReleaseReservesArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

    let contract_id = ContractId::from_str(args.contract_id.as_str())?;
    transaction.contract_id(contract_id);
    transaction.function("releaseAsset");

    let mut params = ContractFunctionParameters::new();

    params.add_address(args.user.as_str());
    params.add_string(args.symbol.as_str());
    let burn_amount = num_bigint::BigUint::from(args.burn_amount);
    params.add_uint256(burn_amount);
    let release_amount = num_bigint::BigUint::from(args.release_amount);
    params.add_uint256(release_amount);
    transaction.function_parameters(params.to_bytes(Some("releaseReserves")));

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}
