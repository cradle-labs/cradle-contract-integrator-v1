use crate::utils::functions::asset_manager::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, ContractId, TokenAssociateTransaction};
use tokio::time::Duration;
use std::str::FromStr;

pub async fn mint(
    args: MintArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(args.asset_contract.parse::<ContractId>()?);
    transaction.function("mint");

    params.add_uint64(args.amount);
    transaction.function_parameters(params.to_bytes(Some("mint")));

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn burn(
    args: BurnArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(args.asset_contract.parse()?);
    transaction.function("burn");

    params.add_uint64(args.amount);
    transaction.function_parameters(params.to_bytes(Some("burn")));

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn wipe(
    args: WipeArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(args.asset_contract.parse()?);
    transaction.function("wipe");

    params.add_uint64(args.amount);
    params.add_address(&args.account);
    transaction.function_parameters(params.to_bytes(Some("wipe")));

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn airdrop(
    args: AirdropArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(args.asset_contract.parse()?);
    transaction.function("airdropTokens");

    params.add_address(&args.target);
    params.add_uint64(args.amount);
    transaction.function_parameters(params.to_bytes(Some("airdropTokens")));

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn self_associate(
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let params = ContractFunctionParameters::new();

    transaction.contract_id(wallet.account_id.parse()?);
    transaction.function("selfAssociate");

    transaction.function_parameters(params.to_bytes(Some("selfAssociate")));

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn grant_kyc(
    asset_manager: String,
    target: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(asset_manager.as_str())?;
    transaction.contract_id(contract_id);

    params.add_address(target.as_str());
    transaction.function_with_parameters("grantKyc", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn transfer(
    args: TransferArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.contract_id(args.asset_contract.parse()?);
    transaction.function("transferTokens");

    params.add_address(&args.target);
    params.add_uint64(args.amount);
    transaction.function_parameters(params.to_bytes(Some("transferTokens")));

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn token_associate(
    id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = TokenAssociateTransaction::new();

    transaction.account_id(wallet.account_id.parse()?);
    transaction.token_ids(vec![id.as_str().parse()?]);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}
