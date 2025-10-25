use anyhow::Result;
use dialoguer::{Input, Select, Confirm};
use contract_integrator::utils::functions::cradle_account::{
    CradleAccountFunctionInput, CradleAccountFunctionOutput, AssociateTokenArgs, WithdrawArgs,
    UpdateBridgingStatusArgs, TransferAssetArgs, GetTradableBalanceArgs, LockAssetArgs,
    UnLockAssetArgs, AddLoanLockArgs, GetLoanAmountArgs, GetCollateralArgs,
    GetLoanBlockIndexArgs, RemoveLoanLockArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Cradle Account Function")
        .items(&[
            "Associate Token",
            "Withdraw",
            "Update Bridging Status",
            "Transfer Asset",
            "Get Tradable Balance",
            "Lock Asset",
            "Unlock Asset",
            "Add Loan Lock",
            "Get Loan Amount",
            "Get Collateral",
            "Get Loan Block Index",
            "Remove Loan Lock",
        ])
        .interact()?;

    let input = match function_selection {
        0 => {
            // AssociateToken
            let token: String = Input::new()
                .with_prompt("Token Address")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::AssociateToken(
                AssociateTokenArgs {
                    token,
                    account_contract_id,
                },
            ))
        }
        1 => {
            // Withdraw
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Withdraw")
                .interact()?;
            let to: String = Input::new()
                .with_prompt("Recipient Address")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::Withdraw(WithdrawArgs {
                asset,
                amount,
                to,
                account_contract_id,
            }))
        }
        2 => {
            // UpdateBridgingStatus
            let new_status: bool = Confirm::new()
                .with_prompt("Enable Bridging?")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::UpdateBridgingStatus(
                UpdateBridgingStatusArgs {
                    new_status,
                    account_contract_id,
                },
            ))
        }
        3 => {
            // TransferAsset
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Transfer")
                .interact()?;
            let to: String = Input::new()
                .with_prompt("Recipient Address")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::TransferAsset(
                TransferAssetArgs {
                    asset,
                    amount,
                    to,
                    account_contract_id,
                },
            ))
        }
        4 => {
            // GetTradableBalance
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::GetTradableBalance(
                GetTradableBalanceArgs {
                    asset,
                    account_contract_id,
                },
            ))
        }
        5 => {
            // LockAsset
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Lock")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::LockAsset(
                LockAssetArgs {
                    asset,
                    amount,
                    account_contract_id,
                },
            ))
        }
        6 => {
            // UnLockAsset
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Unlock")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::UnLockAsset(
                UnLockAssetArgs {
                    asset,
                    amount,
                    account_contract_id,
                },
            ))
        }
        7 => {
            // AddLoanLock
            let lender: String = Input::new()
                .with_prompt("Lender Address")
                .interact()?;
            let collateral: String = Input::new()
                .with_prompt("Collateral Address")
                .interact()?;
            let loan_amount: u64 = Input::new()
                .with_prompt("Loan Amount")
                .interact()?;
            let collateral_amount: u64 = Input::new()
                .with_prompt("Collateral Amount")
                .interact()?;
            let borrow_index: u64 = Input::new()
                .with_prompt("Borrow Index")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::AddLoanLock(
                AddLoanLockArgs {
                    lender,
                    collateral,
                    loan_amount,
                    collateral_amount,
                    borrow_index,
                    account_contract_id,
                },
            ))
        }
        8 => {
            // GetLoanAmount
            let lender: String = Input::new()
                .with_prompt("Lender Address")
                .interact()?;
            let collateral: String = Input::new()
                .with_prompt("Collateral Address")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::GetLoanAmount(
                GetLoanAmountArgs {
                    lender,
                    collateral,
                    account_contract_id,
                },
            ))
        }
        9 => {
            // GetCollateral
            let lender: String = Input::new()
                .with_prompt("Lender Address")
                .interact()?;
            let collateral: String = Input::new()
                .with_prompt("Collateral Address")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::GetCollateral(
                GetCollateralArgs {
                    lender,
                    collateral,
                    account_contract_id,
                },
            ))
        }
        10 => {
            // GetLoanBlockIndex
            let lender: String = Input::new()
                .with_prompt("Lender Address")
                .interact()?;
            let collateral: String = Input::new()
                .with_prompt("Collateral Address")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::GetLoanBlockIndex(
                GetLoanBlockIndexArgs {
                    lender,
                    collateral,
                    account_contract_id,
                },
            ))
        }
        11 => {
            // RemoveLoanLock
            let lender: String = Input::new()
                .with_prompt("Lender Address")
                .interact()?;
            let collateral: String = Input::new()
                .with_prompt("Collateral Address")
                .interact()?;
            let loan_amount: u64 = Input::new()
                .with_prompt("Loan Amount")
                .interact()?;
            let collateral_amount: u64 = Input::new()
                .with_prompt("Collateral Amount")
                .interact()?;
            let borrow_index: u64 = Input::new()
                .with_prompt("Borrow Index")
                .interact()?;
            let account_contract_id: String = Input::new()
                .with_prompt("Account Contract ID")
                .interact()?;

            ContractCallInput::CradleAccount(CradleAccountFunctionInput::RemoveLoanLock(
                RemoveLoanLockArgs {
                    lender,
                    collateral,
                    loan_amount,
                    collateral_amount,
                    borrow_index,
                    account_contract_id,
                },
            ))
        }
        _ => panic!("Invalid selection"),
    };

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response
    match res {
        ContractCallOutput::CradleAccount(output) => {
            match output {
                CradleAccountFunctionOutput::AssociateToken(result) => {
                    println!("✓ Token Associated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::Withdraw(result) => {
                    println!("✓ Withdrawal Successful");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::UpdateBridgingStatus(result) => {
                    println!("✓ Bridging Status Updated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::TransferAsset(result) => {
                    println!("✓ Asset Transferred");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::GetTradableBalance(result) => {
                    println!("✓ Tradable Balance Retrieved");
                    if let Some(info) = result.output {
                        println!("Tradable Balance: {}", info.tradable_balance);
                    }
                }
                CradleAccountFunctionOutput::LockAsset(result) => {
                    println!("✓ Asset Locked");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::UnLockAsset(result) => {
                    println!("✓ Asset Unlocked");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::AddLoanLock(result) => {
                    println!("✓ Loan Lock Added");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::GetLoanAmount(result) => {
                    println!("✓ Loan Amount Retrieved");
                    if let Some(info) = result.output {
                        println!("Loan Amount: {}", info.loan_amount);
                    }
                }
                CradleAccountFunctionOutput::GetCollateral(result) => {
                    println!("✓ Collateral Amount Retrieved");
                    if let Some(info) = result.output {
                        println!("Collateral Amount: {}", info.collateral_amount);
                    }
                }
                CradleAccountFunctionOutput::GetLoanBlockIndex(result) => {
                    println!("✓ Loan Block Index Retrieved");
                    if let Some(info) = result.output {
                        println!("Block Index: {}", info.block_index);
                    }
                }
                CradleAccountFunctionOutput::RemoveLoanLock(result) => {
                    println!("✓ Loan Lock Removed");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                CradleAccountFunctionOutput::Deposit(_result) => {
                    println!("Deposit is only meant to be called on the frontend");
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
