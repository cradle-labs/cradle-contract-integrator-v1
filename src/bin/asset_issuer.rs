use std::env;
use anyhow::Result;
use dialoguer::{Input, Select};
use hedera::ContractId;
use contract_integrator::utils::functions::asset_issuer::{
    AssetIssuerFunctionsInput, AssetIssuerFunctionsOutput, CreateAssetArgs, LockReservesArgs,
    ReleaseAssetArgs, LockAssetArgs, ReleaseReservesArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which asset issuer contract to interact with
    let issuer_selection = Select::new()
        .with_prompt("Select Asset Issuer Contract")
        .items(&["Bridged Asset Issuer", "Native Asset Issuer"])
        .interact()?;

    let issuer_contract_id: ContractId = if issuer_selection == 0 {
        env::var("BRIDGED_ASSET_ISSUER_CONTRACT_ID")?.parse()?
    } else {
        env::var("NATIVE_ASSET_ISSUER_CONTRACT_ID")?.parse()?
    };

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Function")
        .items(&[
            "Create Asset",
            "Lock Reserves",
            "Release Asset",
            "Lock Asset",
            "Release Reserves",
        ])
        .interact()?;

    let input = match function_selection {
        0 => {
            // CreateAsset
            let name: String = Input::new()
                .with_prompt("Asset Name")
                .interact()?;
            let symbol: String = Input::new()
                .with_prompt("Asset Symbol")
                .interact()?;
            let acl_contract: ContractId = Input::new()
                .with_prompt("ACL Contract ID")
                .default(env::var("ACCESS_CONTROLLER_CONTRACT_ID").unwrap_or_default().parse()?)
                .interact()?;
            let allow_list: u64 = Input::new()
                .with_prompt("Allow List Value")
                .default(1)
                .interact()?;

            ContractCallInput::BridgedAssetIssuer(
                AssetIssuerFunctionsInput::CreateAsset(CreateAssetArgs {
                    name,
                    symbol,
                    acl_contract: acl_contract.to_solidity_address()?,
                    allow_list,
                    contract_id: issuer_contract_id.to_solidity_address()?,
                }),
            )
        }
        1 => {
            // LockReserves
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Lock")
                .interact()?;

            ContractCallInput::BridgedAssetIssuer(
                AssetIssuerFunctionsInput::LockReserves(LockReservesArgs {
                    user,
                    amount,
                    contract_id: issuer_contract_id.to_solidity_address()?,
                }),
            )
        }
        2 => {
            // ReleaseAsset
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let symbol: String = Input::new()
                .with_prompt("Asset Symbol")
                .interact()?;
            let mint_amount: u64 = Input::new()
                .with_prompt("Mint Amount")
                .interact()?;
            let unlock_amount: u64 = Input::new()
                .with_prompt("Unlock Amount")
                .interact()?;

            ContractCallInput::BridgedAssetIssuer(
                AssetIssuerFunctionsInput::ReleaseAsset(ReleaseAssetArgs {
                    user,
                    symbol,
                    mint_amount,
                    unlock_amount,
                    contract_id: issuer_contract_id.to_solidity_address()?,
                }),
            )
        }
        3 => {
            // LockAsset
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Lock")
                .interact()?;

            ContractCallInput::BridgedAssetIssuer(
                AssetIssuerFunctionsInput::LockAsset(LockAssetArgs {
                    user,
                    asset,
                    amount,
                    contract_id: issuer_contract_id.to_solidity_address()?,
                }),
            )
        }
        4 => {
            // ReleaseReserves
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let symbol: String = Input::new()
                .with_prompt("Asset Symbol")
                .interact()?;
            let burn_amount: u64 = Input::new()
                .with_prompt("Burn Amount")
                .interact()?;
            let release_amount: u64 = Input::new()
                .with_prompt("Release Amount")
                .interact()?;

            ContractCallInput::BridgedAssetIssuer(
                AssetIssuerFunctionsInput::ReleaseReserves(ReleaseReservesArgs {
                    user,
                    symbol,
                    burn_amount,
                    release_amount,
                    contract_id: issuer_contract_id.to_solidity_address()?,
                }),
            )
        }
        _ => panic!("Invalid selection"),
    };

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response based on the function called
    match res {
        ContractCallOutput::BridgedAssetIssuer(output) => {
            match output {
                AssetIssuerFunctionsOutput::CreateAsset(result) => {
                    println!("✓ Asset Created");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::LockReserves(result) => {
                    println!("✓ Reserves Locked");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::ReleaseAsset(result) => {
                    println!("✓ Asset Released");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::LockAsset(result) => {
                    println!("✓ Asset Locked");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::ReleaseReserves(result) => {
                    println!("✓ Reserves Released");
                    println!("Transaction ID: {}", result.transaction_id);
                }
            }
        }
        ContractCallOutput::NativeAssetIssuer(output) => {
            match output {
                AssetIssuerFunctionsOutput::CreateAsset(result) => {
                    println!("✓ Asset Created");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::LockReserves(result) => {
                    println!("✓ Reserves Locked");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::ReleaseAsset(result) => {
                    println!("✓ Asset Released");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::LockAsset(result) => {
                    println!("✓ Asset Locked");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetIssuerFunctionsOutput::ReleaseReserves(result) => {
                    println!("✓ Reserves Released");
                    println!("Transaction ID: {}", result.transaction_id);
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
