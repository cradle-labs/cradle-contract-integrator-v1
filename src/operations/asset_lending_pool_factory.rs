use crate::utils::functions::asset_lending_pool_factory::*;
use crate::utils::functions::commons::get_contract_id_from_evm_address;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use tokio::time::Duration;

pub async fn create_pool(
    args: CreatePoolArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<CreatePoolResults>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut params = ContractFunctionParameters::new();

    let mut transaction = ContractExecuteTransaction::new();
    transaction.contract_id(contract_ids.asset_lending_pool_factory);
    transaction.gas(10_000_000);

    params.add_uint64(args.ltv);
    params.add_uint64(args.optimal_utilization);
    params.add_uint64(args.base_rate);
    params.add_uint64(args.slope1);
    params.add_uint64(args.slope2);
    params.add_uint64(args.liquidation_threshold);
    params.add_uint64(args.liquidation_discount);
    params.add_uint64(args.reserve_factor);
    params.add_address(args.lending.clone().as_str());
    params.add_address(args.yield_contract.as_str());
    params.add_string(args.lending_pool.clone());

    transaction.function_with_parameters("createPool", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let transaction_id = response.transaction_id.to_string();

    let record = response
        .get_record(&wallet.client)
        .await?
        .contract_function_result
        .ok_or_else(|| anyhow!("Failed to retrieve result"))?;

    let pool_address = record
        .get_address(0)
        .ok_or_else(|| anyhow!("Pool address not found"))?;

    let pool_id = get_contract_id_from_evm_address(pool_address.as_str()).await?;

    let output = FunctionCallOutput {
        transaction_id,
        output: Some(CreatePoolResults {
            address: pool_address,
            contract_id: pool_id.to_string(),
        }),
    };

    Ok(output)
}

pub async fn get_pool(
    args: GetPoolByName,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetPoolResult>> {
    let contract_ids = wallet.get_contract_ids()?;
    let mut params = ContractFunctionParameters::new();
    let mut transaction = ContractCallQuery::new();
    transaction.contract_id(contract_ids.asset_lending_pool_factory);
    transaction.gas(5_000_000);
    params.add_string(args.name.clone());

    transaction.function_with_parameters("getPool", &params);

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?
        .get_address(0);

    if let Some(pool_id) = response {
        let output = FunctionCallOutput {
            transaction_id: "".to_string(),
            output: Some(GetPoolResult { address: pool_id }),
        };

        return Ok(output);
    }

    Err(anyhow!("Failed to get pool address"))
}
