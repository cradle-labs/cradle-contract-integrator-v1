use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use crate::wallet::wallet::ActionWallet;
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::utils::functions::FunctionCallOutput;
use tokio::time::Duration;

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
    pub lending_pool: String
}

pub struct GetPoolByName {
    pub name: String
}

pub enum AssetLendingPoolFactoryFunctionInput {
    CreatePool(CreatePoolArgs),
    GetPool(GetPoolByName)
}


pub struct GetPoolResult {
    pub address: String
}

pub enum AssetLendingPoolFactoryFunctionOutput {
    CreatePool(FunctionCallOutput<()>),
    GetPool(FunctionCallOutput<GetPoolResult>)
}


impl ContractFunctionProcessor<AssetLendingPoolFactoryFunctionOutput> for AssetLendingPoolFactoryFunctionInput {
    async fn process(&self, wallet: &mut ActionWallet) -> anyhow::Result<AssetLendingPoolFactoryFunctionOutput> {

        let contract_ids = wallet.get_contract_ids()?;
        let mut params = ContractFunctionParameters::new();
        match self {
            AssetLendingPoolFactoryFunctionInput::CreatePool(args)=> {
                let mut transaction = ContractExecuteTransaction::new();
                transaction.contract_id(contract_ids.asset_lending_pool_factory);

                params.add_uint64(args.ltv);
                params.add_uint64(args.optimal_utilization);
                params.add_uint64(args.base_rate);
                params.add_uint64(args.slope1);
                params.add_uint64(args.slope2);
                params.add_uint64(args.liquidation_threshold);
                params.add_uint64(args.liquidation_discount);
                params.add_uint64(args.reserve_factor);
                params.add_string(args.lending);
                params.add_address(args.yield_contract.as_str());
                params.add_string(args.lending_pool);

                transaction.function_with_parameters("createPool", &params);

                let transaction_id = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?.transaction_id.to_string();

                let output = FunctionCallOutput {
                    transaction_id,
                    output: None
                };

                Ok(AssetLendingPoolFactoryFunctionOutput::CreatePool(output))
            },
            AssetLendingPoolFactoryFunctionInput::GetPool(args)=>{
                let mut transaction = ContractCallQuery::new();
                transaction.contract_id(contract_ids.asset_lending_pool_factory);

                params.add_string(args.name);

                transaction.function_with_parameters("getPool", &params);

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?.get_address(0);


                if let Some(pool_id) = response {
                    let output = FunctionCallOutput {
                        transaction_id: "".to_string(),
                        output: Some(GetPoolResult {
                            address: pool_id
                        })
                    };


                    return Ok(AssetLendingPoolFactoryFunctionOutput::GetPool(output));
                }

                Err(anyhow!("Failed to get pool address"))
            }
        }
    }
}

