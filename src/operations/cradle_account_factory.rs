use crate::utils::functions::cradle_account_factory::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use tokio::time::Duration;

pub async fn create_account(
    args: CreateAccountInputArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<AddressOutput>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.cradle_account_factory_contract_id);
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.function("createAccount");
    params.add_string(&args.controller);
    params.add_uint64(args.account_allow_list.parse()?);

    transaction.function_parameters(params.to_bytes(Some("createAccount")));

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let record = response.get_record(&wallet.client).await?;
    let result = record
        .contract_function_result
        .ok_or_else(|| anyhow::anyhow!("Failed to find contract result"))?;
    let account_address = result
        .get_address(0)
        .ok_or_else(|| anyhow::anyhow!("Failed to find account address"))?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: Some(AddressOutput { account_address }),
    };

    Ok(output)
}

pub async fn create_account_for_user(
    args: CreateAccountForUserInputArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<AddressOutput>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.cradle_account_factory_contract_id);
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    transaction.function("createAccountForUser");
    params.add_string(&args.controller);
    params.add_address(&args.user);
    params.add_uint64(args.account_allow_list.parse()?);

    transaction.function_parameters(params.to_bytes(Some("createAccountForUser")));

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;
    let record = response.get_record(&wallet.client).await?;
    let result = record
        .contract_function_result
        .ok_or_else(|| anyhow::anyhow!("Failed to find contract result"))?;
    let account_address = result
        .get_address(0)
        .ok_or_else(|| anyhow::anyhow!("Failed to find account address"))?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: Some(AddressOutput { account_address }),
    };

    Ok(output)
}

pub async fn get_account_by_controller(
    args: GetAccountByControllerInputArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<AddressOutput>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.contract_id(contract_ids.cradle_account_factory_contract_id);
    query_transaction.gas(5_000_000);
    let mut params = ContractFunctionParameters::new();

    query_transaction.function("getAccountByController");

    params.add_string(&args.controller);

    query_transaction.function_parameters(params.to_bytes(Some("getAccountByController")));

    let response = query_transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;
    let account_address = response.get_address(0).unwrap();

    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(AddressOutput { account_address }),
    };

    Ok(output)
}
