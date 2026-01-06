use std::env;
use anyhow::{Result, anyhow};
use dialoguer::{Input, Select};
use hedera::ContractId;
use contract_integrator::utils::functions::bridged_asset_issuer_factory::{
    BridgedAssetIssuerFactoryFunctionsInput, CreateBridgedAssetIssuerArgs,
    BridgedAssetIssuerFactoryFunctionsOutput,
};
use contract_integrator::utils::functions::native_asset_issuer_factory::{
    NativeAssetIssuerFactoryFunctionsInput, CreateNativeAssetIssuerArgs,
    NativeAssetIssuerFactoryFunctionsOutput,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which factory contract to interact with
    let factory_selection = Select::new()
        .with_prompt("Select Factory Contract")
        .items(&["Bridged Asset Issuer Factory", "Native Asset Issuer Factory"])
        .interact()?;

    match factory_selection {
        0 => {
             // Bridged Asset Issuer Factory
             let contract_id: ContractId = env::var("BRIDGED_ASSET_ISSUER_FACTORY_CONTRACT_ID")
                .map_err(|_| anyhow!("BRIDGED_ASSET_ISSUER_FACTORY_CONTRACT_ID not found in env"))?
                .parse()?;

             // Select function (currently only one)
             let function_selection = Select::new()
                .with_prompt("Select Function")
                .items(&["Create Bridged Asset Issuer"])
                .interact()?;
            
            let input = match function_selection {
                0 => {
                    let treasury: String = Input::new()
                        .with_prompt("Treasury Address")
                        .interact()?;
                    let acl_contract: ContractId = Input::new()
                        .with_prompt("ACL Contract ID")
                        .default(env::var("ACCESS_CONTROLLER_CONTRACT_ID").unwrap_or_default().parse()?)
                        .interact()?;
                    let allow_list: u64 = Input::new()
                        .with_prompt("Allow List Value")
                        .default(1)
                        .interact()?;
                    let reserve_token: String = Input::new()
                        .with_prompt("Reserve Token Address")
                        .interact()?;

                    ContractCallInput::BridgedAssetIssuerFactory(
                        BridgedAssetIssuerFactoryFunctionsInput::CreateBridgedAssetIssuer(CreateBridgedAssetIssuerArgs {
                            treasury,
                            acl_contract: acl_contract.to_solidity_address()?,
                            allow_list,
                            reserve_token,
                            contract_id: contract_id.to_string(),
                        })
                    )
                }
                _ => panic!("Invalid selection"),
            };

            let res = wallet.execute(input).await?;

            match res {
                ContractCallOutput::BridgedAssetIssuerFactory(output) => {
                    match output {
                        BridgedAssetIssuerFactoryFunctionsOutput::CreateBridgedAssetIssuer(result) => {
                             println!("✓ Bridged Asset Issuer Created");
                             println!("ISSUER ADDRESS: {}", result.output.as_ref().unwrap().issuer_address);
                             println!("Transaction ID: {}", result.transaction_id);
                        }
                    }
                }
                _ => println!("Unexpected contract response"),
            }
        }
        1 => {
             // Native Asset Issuer Factory
             let contract_id: ContractId = env::var("NATIVE_ASSET_ISSUER_FACTORY_CONTRACT_ID")
                .map_err(|_| anyhow!("NATIVE_ASSET_ISSUER_FACTORY_CONTRACT_ID not found in env"))?
                .parse()?;

             // Select function (currently only one)
             let function_selection = Select::new()
                .with_prompt("Select Function")
                .items(&["Create Native Asset Issuer"])
                .interact()?;
            
            let input = match function_selection {
                0 => {
                    let treasury: String = Input::new()
                        .with_prompt("Treasury Address")
                        .interact()?;
                    let acl_contract: ContractId = Input::new()
                        .with_prompt("ACL Contract ID")
                        .default(env::var("ACCESS_CONTROLLER_CONTRACT_ID").unwrap_or_default().parse()?)
                        .interact()?;
                    let allow_list: u64 = Input::new()
                        .with_prompt("Allow List Value")
                        .default(1)
                        .interact()?;
                    let reserve_token: String = Input::new()
                        .with_prompt("Reserve Token Address")
                        .interact()?;

                    ContractCallInput::NativeAssetIssuerFactory(
                        NativeAssetIssuerFactoryFunctionsInput::CreateNativeAssetIssuer(CreateNativeAssetIssuerArgs {
                            treasury,
                            acl_contract: acl_contract.to_solidity_address()?,
                            allow_list,
                            reserve_token,
                            contract_id: contract_id.to_string(),
                        })
                    )
                }
                _ => panic!("Invalid selection"),
            };

            let res = wallet.execute(input).await?;

            match res {
                ContractCallOutput::NativeAssetIssuerFactory(output) => {
                    match output {
                        NativeAssetIssuerFactoryFunctionsOutput::CreateNativeAssetIssuer(result) => {
                             println!("✓ Native Asset Issuer Created");
                             println!("ISSUER ADDRESS: {}", result.output.as_ref().unwrap().issuer_address);
                             println!("Transaction ID: {}", result.transaction_id);
                        }
                    }
                }
                _ => println!("Unexpected contract response"),
            }
        }
        _ => panic!("Invalid selection"),
    }

    Ok(())
}
