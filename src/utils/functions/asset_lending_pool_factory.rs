use crate::utils::functions::FunctionCallOutput;
use crate::utils::functions::commons::{
    ContractFunctionProcessor, get_contract_id_from_evm_address,
};
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePoolArgs {
    pub ltv: u64,
    pub optimal_utilization: u64,
    pub base_rate: u64,
    pub slope1: u64,
    pub slope2: u64,
    pub liquidation_threshold: u64,
    pub liquidation_discount: u64,
    pub reserve_factor: u64,
    pub lending: String,
    pub yield_contract: String,
    pub lending_pool: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPoolByName {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetLendingPoolFactoryFunctionInput {
    CreatePool(CreatePoolArgs),
    GetPool(GetPoolByName),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPoolResult {
    pub address: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreatePoolResults {
    pub address: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetLendingPoolFactoryFunctionOutput {
    CreatePool(FunctionCallOutput<CreatePoolResults>),
    GetPool(FunctionCallOutput<GetPoolResult>),
}

impl ContractFunctionProcessor<AssetLendingPoolFactoryFunctionOutput>
    for AssetLendingPoolFactoryFunctionInput
{
    async fn process(
        &self,
        wallet: &mut ActionWallet,
    ) -> anyhow::Result<AssetLendingPoolFactoryFunctionOutput> {
        let contract_ids = wallet.get_contract_ids()?;
        let mut params = ContractFunctionParameters::new();
        match self {
            AssetLendingPoolFactoryFunctionInput::CreatePool(args) => {
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

                Ok(AssetLendingPoolFactoryFunctionOutput::CreatePool(output))
            }
            AssetLendingPoolFactoryFunctionInput::GetPool(args) => {
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

                    return Ok(AssetLendingPoolFactoryFunctionOutput::GetPool(output));
                }

                Err(anyhow!("Failed to get pool address"))
            }
        }
    }
}
