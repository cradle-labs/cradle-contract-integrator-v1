use std::env;
use anyhow::Result;
use dialoguer::{Input, Select};
use hedera::ContractId;
use contract_integrator::utils::functions::asset_manager::{AssetManagerFunctionInput, AssetManagerFunctionOutput, MintArgs, BurnArgs, WipeArgs, AirdropArgs, TransferArgs};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Asset Manager Function")
        .items(&["Mint", "Burn", "Wipe", "Airdrop", "Self Associate", "TransferTokens", "Token Associate", "Grant KYC"])
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
                asset_contract: asset_contract.to_string(),
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

            println!("Amount {}", amount);

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
        5 => {
            // Transfer Tokens
            let asset_contract: String = Input::new()
                .with_prompt("Asset Contract ID")
                .interact()?;
            let target: String = Input::new()
                .with_prompt("Target Address for Transfer")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Transfer")
                .interact()?;

            ContractCallInput::AssetManager(AssetManagerFunctionInput::Transfer(TransferArgs {
                asset_contract,
                target,
                amount,
            }))
        }
        6 => {
            // Token Associate
            let token_contract: String = Input::new()
                .with_prompt("Token Contract ID")
                .interact()?;

            ContractCallInput::AssetManager(AssetManagerFunctionInput::TokenAssociate(token_contract))
        }
        7 => {
            // Grant KYC
            let token_contract: String = Input::new()
                .with_prompt("Token Contract ID")
                .interact()?;
            let account: String = Input::new()
                .with_prompt("Account Address to Grant KYC")
                .interact()?;

            ContractCallInput::AssetManager(AssetManagerFunctionInput::GrantKYC(token_contract, account))
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
                AssetManagerFunctionOutput::Transfer(result) => {
                    println!("✓ Tokens Transferred");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetManagerFunctionOutput::TokenAssociate(result)=> {
                    println!("✓ Token Associated");
                    println!("Transaction ID: {}", result.transaction_id);
                },
                AssetManagerFunctionOutput::GrantKYC(result) => {
                    println!("✓ KYC Granted");
                    println!("Transaction ID: {}", result.transaction_id);
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
