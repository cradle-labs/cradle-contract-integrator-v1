use std::env;
use anyhow::Result;
use dialoguer::{Input, Select};
use hedera::ContractId;
use contract_integrator::utils::functions::asset_lending_pool_factory::{
    AssetLendingPoolFactoryFunctionInput, AssetLendingPoolFactoryFunctionOutput, CreatePoolArgs,
    GetPoolByName,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Asset Lending Pool Factory Function")
        .items(&["Create Pool", "Get Pool"])
        .interact()?;

    let input = match function_selection {
        0 => {
            // CreatePool
            let ltv: u64 = Input::new()
                .with_prompt("LTV (e.g., 7500 for 75%)")
                .default(7500)
                .interact()?;
            let optimal_utilization: u64 = Input::new()
                .with_prompt("Optimal Utilization (e.g., 8000 for 80%)")
                .default(8000)
                .interact()?;
            let base_rate: u64 = Input::new()
                .with_prompt("Base Rate (e.g., 100 for 1%)")
                .default(100)
                .interact()?;
            let slope1: u64 = Input::new()
                .with_prompt("Slope1 (e.g., 4000 for 40%)")
                .default(4000)
                .interact()?;
            let slope2: u64 = Input::new()
                .with_prompt("Slope2 (e.g., 6000 for 60%)")
                .default(6000)
                .interact()?;
            let liquidation_threshold: u64 = Input::new()
                .with_prompt("Liquidation Threshold (e.g., 8500 for 85%)")
                .default(8500)
                .interact()?;
            let liquidation_discount: u64 = Input::new()
                .with_prompt("Liquidation Discount (e.g., 500 for 5%)")
                .default(500)
                .interact()?;
            let reserve_factor: u64 = Input::new()
                .with_prompt("Reserve Factor (e.g., 1000 for 10%)")
                .default(1000)
                .interact()?;
            let lending: String = Input::new()
                .with_prompt("Lending Asset ID")
                .default(env::var("LENDING").unwrap_or_default())
                .interact()?;
            let yield_contract: ContractId = Input::new()
                .with_prompt("Yield Contract ID")
                .interact()?;
            let lending_pool: String = Input::new()
                .with_prompt("Lending Pool Name")
                .default(env::var("LENDING_POOL").unwrap_or_default())
                .interact()?;

            ContractCallInput::AssetLendingPoolFactory(
                AssetLendingPoolFactoryFunctionInput::CreatePool(CreatePoolArgs {
                    ltv,
                    optimal_utilization,
                    base_rate,
                    slope1,
                    slope2,
                    liquidation_threshold,
                    liquidation_discount,
                    reserve_factor,
                    lending,
                    yield_contract: yield_contract.to_solidity_address()?,
                    lending_pool,
                }),
            )
        }
        1 => {
            // GetPool
            let name: String = Input::new()
                .with_prompt("Pool Name to Query")
                .interact()?;

            ContractCallInput::AssetLendingPoolFactory(
                AssetLendingPoolFactoryFunctionInput::GetPool(GetPoolByName { name }),
            )
        }
        _ => panic!("Invalid selection"),
    };

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response based on the function called
    match res {
        ContractCallOutput::AssetLendingPoolFactory(output) => {
            match output {
                AssetLendingPoolFactoryFunctionOutput::CreatePool(result) => {
                    println!("✓ Lending Pool Created");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFactoryFunctionOutput::GetPool(result) => {
                    println!("✓ Pool Retrieved");
                    if let Some(pool_info) = result.output {
                        println!("Pool Address: {}", pool_info.address);
                    }
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
