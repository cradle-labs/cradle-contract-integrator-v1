use crate::utils::functions::access_controller::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use tokio::time::Duration;

pub async fn has_access(
    args: AccessControllerArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<HasAccessOutput>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.contract_id(contract_ids.access_controller_contract_id);
    query_transaction.gas(1_000_000);

    query_transaction.function("hasAccess");
    let mut params = ContractFunctionParameters::new();

    params.add_uint64(args.level);
    params.add_address(args.account.as_str());

    query_transaction.function_parameters(params.to_bytes(Some("hasAccess")));

    let response = query_transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let has_access = response.get_bool(0).unwrap();

    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(HasAccessOutput { has_access }),
    };
    Ok(output)
}

pub async fn grant_access(
    args: AccessControllerArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.access_controller_contract_id);
    transaction.gas(10_000_000);

    transaction.function("grantAccess");
    let mut params = ContractFunctionParameters::new();

    params.add_uint64(args.level);
    params.add_address(args.account.as_str());

    transaction.function_parameters(params.to_bytes(Some("grantAccess")));

    let response = transaction.execute(&mut wallet.client).await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn revoke_access(
    args: AccessControllerArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.access_controller_contract_id);
    transaction.gas(10_000_000);

    transaction.function("revokeAccess");
    let mut params = ContractFunctionParameters::new();

    params.add_uint64(args.level);
    params.add_address(args.account.as_str());

    transaction.function_parameters(params.to_bytes(Some("revokeAccess")));

    let response = transaction.execute(&mut wallet.client).await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn grant_access_batch(
    args: GrantAccessBatchArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.access_controller_contract_id);
    transaction.gas(10_000_000);

    transaction.function("grantAccessBatch");
    let mut params = ContractFunctionParameters::new();

    params.add_uint64(args.level);
    let addresses: Vec<&str> = args.accounts.iter().map(|s| s.as_str()).collect();
    params.add_address_array(&addresses);

    transaction.function_parameters(params.to_bytes(Some("grantAccessBatch")));

    let response = transaction.execute(&mut wallet.client).await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn clear_level(
    args: ClearLevelArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.access_controller_contract_id);
    transaction.gas(10_000_000);

    transaction.function("clearLevel");
    let mut params = ContractFunctionParameters::new();

    params.add_uint64(args.level);
    transaction.function_parameters(params.to_bytes(Some("clearLevel")));

    let response = transaction.execute(&mut wallet.client).await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn get_level(
    args: GetLevelArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<Vec<String>>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.contract_id(contract_ids.access_controller_contract_id);
    query_transaction.gas(1_000_000);

    query_transaction.function("getLevel");
    let mut params = ContractFunctionParameters::new();

    params.add_uint64(args.level);

    query_transaction.function_parameters(params.to_bytes(Some("getLevel")));

    let _response = query_transaction.execute(&mut wallet.client).await?;

    // let receipt = response.get;
    todo!("Seems address array is not supported yet in hedera sdk");
    // let output = FunctionCallOutput {
    //     transaction_id: receipt.transaction_id.unwrap().to_string(),
    //     output: None
    // };
    //
    // Ok(output)
}

pub async fn rotate_admin(
    args: RotateAdminArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.access_controller_contract_id);
    transaction.gas(10_000_000);

    transaction.function("rotateLevel0Key");

    let mut params = ContractFunctionParameters::new();
    params.add_address(args.old_key.as_str());
    params.add_address(args.new_key.as_str());

    transaction.function_parameters(params.to_bytes(Some("rotateLevel0Key")));

    let response = transaction.execute(&mut wallet.client).await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}
