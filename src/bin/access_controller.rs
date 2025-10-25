use anyhow::Result;
use dialoguer::{Input, Select};
use contract_integrator::utils::functions::access_controller::{
    AccessControllerFunctionsInput, AccessControllerFunctionsOutput, AccessControllerArgs,
    GrantAccessBatchArgs, ClearLevelArgs, GetLevelArgs, RotateAdminArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Access Controller Function")
        .items(&[
            "Has Access",
            "Grant Access",
            "Revoke Access",
            "Grant Access Batch",
            "Clear Level",
            "Get Level",
            "Rotate Admin",
        ])
        .interact()?;

    let input = match function_selection {
        0 => {
            // HasAccess
            let level: u64 = Input::new()
                .with_prompt("Access Level")
                .interact()?;
            let account: String = Input::new()
                .with_prompt("Account Address")
                .interact()?;

            ContractCallInput::AccessController(AccessControllerFunctionsInput::HasAccess(
                AccessControllerArgs { level, account },
            ))
        }
        1 => {
            // GrantAccess
            let level: u64 = Input::new()
                .with_prompt("Access Level to Grant")
                .interact()?;
            let account: String = Input::new()
                .with_prompt("Account Address")
                .interact()?;

            ContractCallInput::AccessController(AccessControllerFunctionsInput::GrantAccess(
                AccessControllerArgs { level, account },
            ))
        }
        2 => {
            // RevokeAccess
            let level: u64 = Input::new()
                .with_prompt("Access Level to Revoke")
                .interact()?;
            let account: String = Input::new()
                .with_prompt("Account Address")
                .interact()?;

            ContractCallInput::AccessController(AccessControllerFunctionsInput::RevokeAccess(
                AccessControllerArgs { level, account },
            ))
        }
        3 => {
            // GrantAccessBatch
            let level: u64 = Input::new()
                .with_prompt("Access Level to Grant")
                .interact()?;
            let accounts_input: String = Input::new()
                .with_prompt("Accounts (comma-separated)")
                .interact()?;
            let accounts: Vec<String> = accounts_input
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            ContractCallInput::AccessController(
                AccessControllerFunctionsInput::GrantAccessBatch(GrantAccessBatchArgs {
                    level,
                    accounts,
                }),
            )
        }
        4 => {
            // ClearLevel
            let level: u64 = Input::new()
                .with_prompt("Level to Clear")
                .interact()?;

            ContractCallInput::AccessController(AccessControllerFunctionsInput::ClearLevel(
                ClearLevelArgs { level },
            ))
        }
        5 => {
            // GetLevel
            let level: u64 = Input::new()
                .with_prompt("Level to Query")
                .interact()?;

            ContractCallInput::AccessController(AccessControllerFunctionsInput::GetLevel(
                GetLevelArgs { level },
            ))
        }
        6 => {
            // RotateAdmin
            let old_key: String = Input::new()
                .with_prompt("Old Admin Key")
                .interact()?;
            let new_key: String = Input::new()
                .with_prompt("New Admin Key")
                .interact()?;

            ContractCallInput::AccessController(AccessControllerFunctionsInput::RotateAdmin(
                RotateAdminArgs { old_key, new_key },
            ))
        }
        _ => panic!("Invalid selection"),
    };

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response
    match res {
        ContractCallOutput::AccessController(output) => {
            match output {
                AccessControllerFunctionsOutput::HasAccess(result) => {
                    println!("✓ Access Check Complete");
                    if let Some(info) = result.output {
                        println!("Has Access: {}", info.has_access);
                    }
                }
                AccessControllerFunctionsOutput::GrantAccess(result) => {
                    println!("✓ Access Granted");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AccessControllerFunctionsOutput::RevokeAccess(result) => {
                    println!("✓ Access Revoked");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AccessControllerFunctionsOutput::GrantAccessBatch(result) => {
                    println!("✓ Batch Access Granted");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AccessControllerFunctionsOutput::ClearLevel(result) => {
                    println!("✓ Level Cleared");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AccessControllerFunctionsOutput::GetLevel(result) => {
                    println!("✓ Level Retrieved");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AccessControllerFunctionsOutput::RotateAdmin(result) => {
                    println!("✓ Admin Rotated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
