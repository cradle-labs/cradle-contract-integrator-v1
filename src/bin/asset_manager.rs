use std::env;
use anyhow::Result;
use dialoguer::{Input, Select};
use hedera::ContractId;
use contract_integrator::utils::functions::asset_manager::{
    AssetManagerFunctionInput, AssetManagerFunctionOutput, MintArgs, BurnArgs, WipeArgs,
    AirdropArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Asset Manager Function")
        .items(&["Mint", "Burn", "Wipe", "Airdrop", "Self Associate"])
        .interact()?;

    let input = match function_selection {
        0 => {
            // Mint
            let asset_contract: ContractId = Input::new()
                .with_prompt("Asset Contract ID")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Mint")
                .interact()?;

            ContractCallInput::AssetManager(AssetManagerFunctionInput::Mint(MintArgs {
                asset_contract: asset_contract.to_solidity_address()?,
                amount,
            }))
        }
        1 => {
            // Burn
            let asset_contract: String = Input::new()
                .with_prompt("Asset Contract ID")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Burn")
                .interact()?;

            ContractCallInput::AssetManager(AssetManagerFunctionInput::Burn(BurnArgs {
                asset_contract,
                amount,
            }))
        }
        2 => {
            // Wipe
            let asset_contract: String = Input::new()
                .with_prompt("Asset Contract ID")
                .interact()?;
            let account: String = Input::new()
                .with_prompt("Account Address to Wipe")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Wipe")
                .interact()?;

            ContractCallInput::AssetManager(AssetManagerFunctionInput::Wipe(WipeArgs {
                asset_contract,
                account,
                amount,
            }))
        }
        3 => {
            // Airdrop
            let asset_contract: String = Input::new()
                .with_prompt("Asset Contract ID")
                .interact()?;
            let target: String = Input::new()
                .with_prompt("Target Address for Airdrop")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Airdrop")
                .interact()?;

            ContractCallInput::AssetManager(AssetManagerFunctionInput::Airdrop(AirdropArgs {
                asset_contract,
                target,
                amount,
            }))
        }
        4 => {
            // Self Associate
            ContractCallInput::AssetManager(AssetManagerFunctionInput::SelfAssociate)
        }
        _ => panic!("Invalid selection"),
    };

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response based on the function called
    match res {
        ContractCallOutput::AssetManager(output) => {
            match output {
                AssetManagerFunctionOutput::Mint(result) => {
                    println!("✓ Asset Minted");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetManagerFunctionOutput::Burn(result) => {
                    println!("✓ Asset Burned");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetManagerFunctionOutput::Wipe(result) => {
                    println!("✓ Account Wiped");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetManagerFunctionOutput::Airdrop(result) => {
                    println!("✓ Airdrop Completed");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetManagerFunctionOutput::SelfAssociate(result) => {
                    println!("✓ Self Associated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                _=>{
                    unimplemented!()
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
