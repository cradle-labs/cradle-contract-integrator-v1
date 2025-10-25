use anyhow::Result;
use dialoguer::{Input, Select};
use contract_integrator::utils::functions::asset_lending::{
    AssetLendingPoolFunctionsInput, AssetLendingPoolFunctionsOutput, CalculateCurrentDebtArgs,
    CalculateCurrentDepositArgs, CalculateHealthFactorArgs, UpdateOracleArgs,
    GetAssetMultiplierArgs, GetUserDepositPositon, GetUserBorrowPosition, GetMaxBorrowAmount,
    IsPositionLiquidatableArgs, DepositArgs, WithdrawArgs, BorrowArgs, RepayArgs,
    LiquidateArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Asset Lending Pool Function")
        .items(&[
            "Get Utilization",
            "Get Borrow Rate",
            "Get Supply Rate",
            "Update Borrow Index",
            "Update Supply Index",
            "Update Indices",
            "Calculate Current Debt",
            "Calculate Current Deposit",
            "Calculate Health Factor",
            "Update Oracle",
            "Get Asset Multiplier",
            "Get User Deposit Position",
            "Get User Borrow Position",
            "Get Max Borrow Amount",
            "Is Position Liquidatable",
            "Get Pool Stats",
            "Deposit",
            "Withdraw",
            "Borrow",
            "Repay",
            "Liquidate",
        ])
        .interact()?;

    let input = match function_selection {
        0 => ContractCallInput::AssetLendingPool(
            AssetLendingPoolFunctionsInput::GetUtilization,
        ),
        1 => ContractCallInput::AssetLendingPool(
            AssetLendingPoolFunctionsInput::GetBorrowRate,
        ),
        2 => ContractCallInput::AssetLendingPool(
            AssetLendingPoolFunctionsInput::GetSupplyRate,
        ),
        3 => ContractCallInput::AssetLendingPool(
            AssetLendingPoolFunctionsInput::UpdateBorrowIndex,
        ),
        4 => ContractCallInput::AssetLendingPool(
            AssetLendingPoolFunctionsInput::UpdateSupplyIndex,
        ),
        5 => ContractCallInput::AssetLendingPool(
            AssetLendingPoolFunctionsInput::UpdateIndices,
        ),
        6 => {
            let user_principal: u64 = Input::new()
                .with_prompt("User Principal")
                .interact()?;
            let user_borrow_index: u64 = Input::new()
                .with_prompt("User Borrow Index")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::CalculateCurrentDebt(CalculateCurrentDebtArgs {
                    user_principal,
                    user_borrow_index,
                }),
            )
        }
        7 => {
            let user_shares: u64 = Input::new()
                .with_prompt("User Shares")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::CalculateCurrentDeposit(
                    CalculateCurrentDepositArgs { user_shares },
                ),
            )
        }
        8 => {
            let collateral_value: u64 = Input::new()
                .with_prompt("Collateral Value")
                .interact()?;
            let borrowed_value: u64 = Input::new()
                .with_prompt("Borrowed Value")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::CalculateHealthFactor(
                    CalculateHealthFactorArgs {
                        collateral_value,
                        borrowed_value,
                    },
                ),
            )
        }
        9 => {
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;
            let multiplier: u64 = Input::new()
                .with_prompt("Multiplier Value")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::UpdateOracle(UpdateOracleArgs {
                    asset,
                    multiplier,
                }),
            )
        }
        10 => {
            let asset: String = Input::new()
                .with_prompt("Asset Address")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::GetAssetMultiplier(GetAssetMultiplierArgs {
                    asset,
                }),
            )
        }
        11 => {
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::GetUserDepositPosition(GetUserDepositPositon {
                    user,
                }),
            )
        }
        12 => {
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let collateral_asset: String = Input::new()
                .with_prompt("Collateral Asset Address")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::GetUserBorrowPosition(GetUserBorrowPosition {
                    user,
                    collateral_asset,
                }),
            )
        }
        13 => {
            let collateral_amount: u64 = Input::new()
                .with_prompt("Collateral Amount")
                .interact()?;
            let collateral_asset: String = Input::new()
                .with_prompt("Collateral Asset Address")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::GetMaxBorrowAmount(GetMaxBorrowAmount {
                    collateral_amount,
                    collateral_asset,
                }),
            )
        }
        14 => {
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let collateral_asset: String = Input::new()
                .with_prompt("Collateral Asset Address")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::IsPositionLiquidatable(
                    IsPositionLiquidatableArgs {
                        user,
                        collateral_asset,
                    },
                ),
            )
        }
        15 => ContractCallInput::AssetLendingPool(
            AssetLendingPoolFunctionsInput::GetPoolStats,
        ),
        16 => {
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let amount: u64 = Input::new()
                .with_prompt("Amount to Deposit")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::Deposit(DepositArgs { user, amount }),
            )
        }
        17 => {
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let yield_token_amount: u64 = Input::new()
                .with_prompt("Yield Token Amount to Withdraw")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::Withdraw(WithdrawArgs {
                    user,
                    yield_token_amount,
                }),
            )
        }
        18 => {
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let collateral_amount: u64 = Input::new()
                .with_prompt("Collateral Amount")
                .interact()?;
            let collateral_asset: String = Input::new()
                .with_prompt("Collateral Asset Address")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::Borrow(BorrowArgs {
                    user,
                    collateral_amount,
                    collateral_asset,
                }),
            )
        }
        19 => {
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let collateralized_asset: String = Input::new()
                .with_prompt("Collateralized Asset Address")
                .interact()?;
            let repay_amount: u64 = Input::new()
                .with_prompt("Repay Amount")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::Repay(RepayArgs {
                    user,
                    collateralized_asset,
                    repay_amount,
                }),
            )
        }
        20 => {
            let liquidator: String = Input::new()
                .with_prompt("Liquidator Address")
                .interact()?;
            let borrower: String = Input::new()
                .with_prompt("Borrower Address")
                .interact()?;
            let dept_to_cover: u64 = Input::new()
                .with_prompt("Debt to Cover")
                .interact()?;
            let collateral_asset: String = Input::new()
                .with_prompt("Collateral Asset Address")
                .interact()?;

            ContractCallInput::AssetLendingPool(
                AssetLendingPoolFunctionsInput::Liquidate(LiquidateArgs {
                    liquidator,
                    borrower,
                    dept_to_cover,
                    collateral_asset,
                }),
            )
        }
        _ => panic!("Invalid selection"),
    };

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response
    match res {
        ContractCallOutput::AssetLendingPool(output) => {
            match output {
                AssetLendingPoolFunctionsOutput::GetUtilization(result) => {
                    println!("✓ Utilization Retrieved");
                    if let Some(info) = result.output {
                        println!("Utilization: {}", info.utilization);
                    }
                }
                AssetLendingPoolFunctionsOutput::GetBorrowRate(result) => {
                    println!("✓ Borrow Rate Retrieved");
                    if let Some(info) = result.output {
                        println!("Borrow Rate: {}", info.borrow_rate);
                    }
                }
                AssetLendingPoolFunctionsOutput::GetSupplyRate(result) => {
                    println!("✓ Supply Rate Retrieved");
                    if let Some(info) = result.output {
                        println!("Supply Rate: {}", info.supply_rate);
                    }
                }
                AssetLendingPoolFunctionsOutput::UpdateBorrowIndex(result) => {
                    println!("✓ Borrow Index Updated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::UpdateSupplyIndex(result) => {
                    println!("✓ Supply Index Updated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::UpdateIndices(result) => {
                    println!("✓ Indices Updated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::CalculateCurrentDebt(result) => {
                    println!("✓ Current Debt Calculated");
                    if let Some(info) = result.output {
                        println!("Current Debt: {}", info.current_debt);
                    }
                }
                AssetLendingPoolFunctionsOutput::CalculateCurrentDeposit(result) => {
                    println!("✓ Current Deposit Calculated");
                    if let Some(info) = result.output {
                        println!("Current Deposit: {}", info.current_deposit);
                    }
                }
                AssetLendingPoolFunctionsOutput::CalculateHealthFactor(result) => {
                    println!("✓ Health Factor Calculated");
                    if let Some(info) = result.output {
                        println!("Health Factor: {}", info.health_factor);
                    }
                }
                AssetLendingPoolFunctionsOutput::UpdateOracle(result) => {
                    println!("✓ Oracle Updated");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::GetAssetMultiplier(result) => {
                    println!("✓ Asset Multiplier Retrieved");
                    if let Some(info) = result.output {
                        println!("Multiplier: {}", info.multiplier);
                    }
                }
                AssetLendingPoolFunctionsOutput::GetUserDepositPosition(result) => {
                    println!("✓ User Deposit Position Retrieved");
                    if let Some(info) = result.output {
                        println!("Yield Token Balance: {}", info.yield_token_balance);
                        println!("Underlying Value: {}", info.underlying_value);
                        println!("Current Supply APY: {}", info.current_supply_apy);
                    }
                }
                AssetLendingPoolFunctionsOutput::GetUserBorrowPosition(result) => {
                    println!("✓ User Borrow Position Retrieved");
                    if let Some(info) = result.output {
                        println!("Principal Borrowed: {}", info.principal_borrowed);
                        println!("Current Debt: {}", info.current_dept);
                        println!("Collateral Amount: {}", info.collateral_amount);
                        println!("Health Factor: {}", info.health_factor);
                        println!("Borrow Index: {}", info.borrow_index);
                    }
                }
                AssetLendingPoolFunctionsOutput::GetMaxBorrowAmount(result) => {
                    println!("✓ Max Borrow Amount Retrieved");
                    if let Some(info) = result.output {
                        println!("Max Borrow Amount: {}", info.max_borrow_amount);
                    }
                }
                AssetLendingPoolFunctionsOutput::IsPositionLiquidatable(result) => {
                    println!("✓ Position Liquidatability Checked");
                    if let Some(info) = result.output {
                        println!("Liquidatable: {}", info.liquidatable);
                        println!("Health Factor: {}", info.health_factor);
                    }
                }
                AssetLendingPoolFunctionsOutput::GetPoolStats(result) => {
                    println!("✓ Pool Stats Retrieved");
                    if let Some(info) = result.output {
                        println!("Total Supplied: {}", info.total_supplied);
                        println!("Total Borrowed: {}", info.total_borrowed);
                        println!("Liquidity: {}", info.liquidity);
                        println!("Utilization: {}", info.utilization);
                        println!("Borrow Rate: {}", info.borrow_rate);
                        println!("Supply Rate: {}", info.supply_rate);
                    }
                }
                AssetLendingPoolFunctionsOutput::Deposit(result) => {
                    println!("✓ Deposit Successful");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::Withdraw(result) => {
                    println!("✓ Withdrawal Successful");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::Borrow(result) => {
                    println!("✓ Borrow Successful");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::Repay(result) => {
                    println!("✓ Repay Successful");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                AssetLendingPoolFunctionsOutput::Liquidate(result) => {
                    println!("✓ Liquidation Successful");
                    println!("Transaction ID: {}", result.transaction_id);
                }
                _ => println!("Query successful"),
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
