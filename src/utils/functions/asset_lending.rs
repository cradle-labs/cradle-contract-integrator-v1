use crate::utils::functions::FunctionCallOutput;
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters, Hbar};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalculateCurrentDebtArgs {
    pub user_principal: u64,
    pub user_borrow_index: u64,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalculateCurrentDepositArgs {
    pub user_shares: u64,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalculateHealthFactorArgs {
    pub collateral_value: u64,
    pub borrowed_value: u64,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOracleArgs {
    pub asset: String,
    pub multiplier: u64,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAssetMultiplierArgs {
    pub asset: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserDepositPositon {
    pub user: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserBorrowPosition {
    pub user: String,
    pub collateral_asset: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMaxBorrowAmount {
    pub collateral_amount: u64,
    pub collateral_asset: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsPositionLiquidatableArgs {
    pub user: String,
    pub collateral_asset: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DepositArgs {
    pub user: String,
    pub amount: u64,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawArgs {
    pub user: String,
    pub yield_token_amount: u64,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BorrowArgs {
    pub user: String,
    pub collateral_amount: u64,
    pub collateral_asset: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepayArgs {
    pub user: String,
    pub collateralized_asset: String,
    pub repay_amount: u64,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidateArgs {
    pub liquidator: String,
    pub borrower: String,
    pub dept_to_cover: u64,
    pub collateral_asset: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetLendingPoolFunctionsInput {
    GetUtilization(String),
    GetBorrowRate(String),
    GetSupplyRate(String),
    UpdateBorrowIndex(String),
    UpdateSupplyIndex(String),
    UpdateIndices(String),
    CalculateCurrentDebt(CalculateCurrentDebtArgs),
    CalculateCurrentDeposit(CalculateCurrentDepositArgs),
    CalculateHealthFactor(CalculateHealthFactorArgs),
    UpdateOracle(UpdateOracleArgs),
    GetAssetMultiplier(GetAssetMultiplierArgs),
    GetUserDepositPosition(GetUserDepositPositon),
    GetUserBorrowPosition(GetUserBorrowPosition),
    GetMaxBorrowAmount(GetMaxBorrowAmount),
    IsPositionLiquidatable(IsPositionLiquidatableArgs),
    GetPoolStats(String),
    Deposit(DepositArgs),
    Withdraw(WithdrawArgs),
    Borrow(BorrowArgs),
    Repay(RepayArgs),
    Liquidate(LiquidateArgs),
    GetReserveAccount(String),
    GetTreasuryAccount(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUtilizationOutput {
    pub utilization: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBorrowRateOutput {
    pub borrow_rate: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSupplyRateOutput {
    pub supply_rate: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalculateCurrentDebtOutput {
    pub current_debt: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalculateCurrentDepositOutput {
    pub current_deposit: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalculateHealthFactorOutput {
    pub health_factor: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAssetMultiplierOutput {
    pub multiplier: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserDepositPositonOutput {
    pub yield_token_balance: u64,
    pub underlying_value: u64,
    pub current_supply_apy: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserBorrowPositionOutput {
    pub principal_borrowed: u64,
    pub current_dept: u64,
    pub collateral_amount: u64,
    pub health_factor: u64,
    pub borrow_index: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMaxBorrowAmountOutput {
    pub max_borrow_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsPositionLiquidatableOutput {
    pub liquidatable: bool,
    pub health_factor: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPoolStatsOutput {
    pub total_supplied: u64,
    pub total_borrowed: u64,
    pub liquidity: u64,
    pub utilization: u64,
    pub borrow_rate: u64,
    pub supply_rate: u64,
    pub borrow_index: u64,
    pub supply_index: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccount {
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BorrowResultArgs {
    pub borrow_index: u64,
    pub borrowed_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepayResultArgs {
    pub collateral_unlocked: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidateResultArgs {
    pub obtained_collateral: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetLendingPoolFunctionsOutput {
    GetUtilization(FunctionCallOutput<GetUtilizationOutput>),
    GetBorrowRate(FunctionCallOutput<GetBorrowRateOutput>),
    GetSupplyRate(FunctionCallOutput<GetSupplyRateOutput>),
    UpdateBorrowIndex(FunctionCallOutput<()>),
    UpdateSupplyIndex(FunctionCallOutput<()>),
    UpdateIndices(FunctionCallOutput<()>),
    CalculateCurrentDebt(FunctionCallOutput<CalculateCurrentDebtOutput>),
    CalculateCurrentDeposit(FunctionCallOutput<CalculateCurrentDepositOutput>),
    CalculateHealthFactor(FunctionCallOutput<CalculateHealthFactorOutput>),
    UpdateOracle(FunctionCallOutput<()>),
    GetAssetMultiplier(FunctionCallOutput<GetAssetMultiplierOutput>),
    GetUserDepositPosition(FunctionCallOutput<GetUserDepositPositonOutput>),
    GetUserBorrowPosition(FunctionCallOutput<GetUserBorrowPositionOutput>),
    GetMaxBorrowAmount(FunctionCallOutput<GetMaxBorrowAmountOutput>),
    IsPositionLiquidatable(FunctionCallOutput<IsPositionLiquidatableOutput>),
    GetPoolStats(FunctionCallOutput<GetPoolStatsOutput>),
    Deposit(FunctionCallOutput<(u64, u64)>),
    Withdraw(FunctionCallOutput<(u64, u64)>),
    Borrow(FunctionCallOutput<BorrowResultArgs>),
    Repay(FunctionCallOutput<RepayResultArgs>),
    Liquidate(FunctionCallOutput<LiquidateResultArgs>),
    GetReserveAccount(FunctionCallOutput<GetAccount>),
    GetTreasuryAccount(FunctionCallOutput<GetAccount>),
}

impl ContractFunctionProcessor<AssetLendingPoolFunctionsOutput> for AssetLendingPoolFunctionsInput {
    async fn process(
        &self,
        wallet: &mut ActionWallet,
    ) -> anyhow::Result<AssetLendingPoolFunctionsOutput> {
        let contract_ids = wallet.get_contract_ids()?;

        let mut transaction = ContractExecuteTransaction::new();

        transaction.gas(10_000_000);
        let mut query_transaction = ContractCallQuery::new();
        query_transaction.gas(5_000_000);
        query_transaction.payment_amount(Hbar::new(10));

        match self {
            AssetLendingPoolFunctionsInput::GetUtilization(contract_id) => {
                query_transaction.contract_id(contract_id.parse()?);
                query_transaction.function("getUtilization");
                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let utilization: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetUtilizationOutput { utilization }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetUtilization(output))
            }
            AssetLendingPoolFunctionsInput::GetBorrowRate(contract_id) => {
                query_transaction.contract_id(contract_id.parse()?);
                query_transaction.function("getBorrowRate");
                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let borrow_rate: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetBorrowRateOutput { borrow_rate }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetBorrowRate(output))
            }
            AssetLendingPoolFunctionsInput::GetSupplyRate(contract_id) => {
                query_transaction.contract_id(contract_id.parse()?);
                query_transaction.function("getSupplyRate");

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let supply_rate: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetSupplyRateOutput { supply_rate }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetSupplyRate(output))
            }
            AssetLendingPoolFunctionsInput::UpdateBorrowIndex(contract_id) => {
                query_transaction.contract_id(contract_id.parse()?);
                transaction.function("updateBorrowIndex");

                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AssetLendingPoolFunctionsOutput::UpdateBorrowIndex(output))
            }
            AssetLendingPoolFunctionsInput::UpdateSupplyIndex(contract_id) => {
                transaction.contract_id(contract_id.parse()?);
                transaction.function("updateSupplyIndex");

                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AssetLendingPoolFunctionsOutput::UpdateSupplyIndex(output))
            }
            AssetLendingPoolFunctionsInput::UpdateIndices(contract_id) => {
                transaction.contract_id(contract_id.parse()?);
                transaction.function("updateIndices");

                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;
                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AssetLendingPoolFunctionsOutput::UpdateIndices(output))
            }
            AssetLendingPoolFunctionsInput::CalculateCurrentDebt(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                query_transaction.function("calculateCurrentDebt");

                let mut params = ContractFunctionParameters::new();

                let user_principal = BigUint::from(args.user_principal);
                let user_borrow_index = BigUint::from(args.user_borrow_index);

                params.add_uint256(user_principal);
                params.add_uint256(user_borrow_index);

                query_transaction
                    .function_parameters(params.to_bytes(Some("calculateCurrentDebt")));

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;
                let current_debt: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(CalculateCurrentDebtOutput { current_debt }),
                };

                Ok(AssetLendingPoolFunctionsOutput::CalculateCurrentDebt(
                    output,
                ))
            }
            AssetLendingPoolFunctionsInput::CalculateCurrentDeposit(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                query_transaction.function("calculateCurrentDeposit");

                let mut params = ContractFunctionParameters::new();
                let user_shares = BigUint::from(args.user_shares);
                params.add_uint256(user_shares);

                query_transaction
                    .function_parameters(params.to_bytes(Some("calculateCurrentDeposit")));

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let current_deposit: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(CalculateCurrentDepositOutput { current_deposit }),
                };

                Ok(AssetLendingPoolFunctionsOutput::CalculateCurrentDeposit(
                    output,
                ))
            }
            AssetLendingPoolFunctionsInput::CalculateHealthFactor(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                query_transaction.function("calculateHealthFactor");

                let mut params = ContractFunctionParameters::new();
                let collateral_value = BigUint::from(args.collateral_value);
                let borrowed_value = BigUint::from(args.borrowed_value);
                params.add_uint256(collateral_value);
                params.add_uint256(borrowed_value);

                query_transaction
                    .function_parameters(params.to_bytes(Some("calculateHealthFactor")));
                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;
                let health_factor: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(CalculateHealthFactorOutput { health_factor }),
                };

                Ok(AssetLendingPoolFunctionsOutput::CalculateHealthFactor(
                    output,
                ))
            }
            AssetLendingPoolFunctionsInput::UpdateOracle(args) => {
                transaction.contract_id(args.contract_id.parse()?);
                transaction.function("updateOracle");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.asset.as_str());
                let multiplier = BigUint::from(args.multiplier);
                params.add_uint256(multiplier);

                transaction.function_parameters(params.to_bytes(Some("updateOracle")));
                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AssetLendingPoolFunctionsOutput::UpdateOracle(output))
            }
            AssetLendingPoolFunctionsInput::GetAssetMultiplier(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                let mut params = ContractFunctionParameters::new();

                params.add_address(args.asset.as_str());
                query_transaction.function_with_parameters("getMultiplier", &params);

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let multiplier: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetAssetMultiplierOutput { multiplier }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetAssetMultiplier(output))
            }
            AssetLendingPoolFunctionsInput::GetUserDepositPosition(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                let mut params = ContractFunctionParameters::new();

                params.add_address(args.user.as_str());
                query_transaction.function_with_parameters("getUserDepositPosition", &params);

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let yield_token_balance: u64 = response.get_u256(0).unwrap().try_into()?;
                let underlying_value: u64 = response.get_u256(1).unwrap().try_into()?;
                let current_supply_apy: u64 = response.get_u256(2).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetUserDepositPositonOutput {
                        yield_token_balance,
                        underlying_value,
                        current_supply_apy,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetUserDepositPosition(
                    output,
                ))
            }
            AssetLendingPoolFunctionsInput::GetUserBorrowPosition(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                let mut params = ContractFunctionParameters::new();
                params.add_address(args.user.as_str());
                params.add_address(args.collateral_asset.as_str());
                query_transaction.function_with_parameters("getUserBorrowPosition", &params);

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;
                let principal_borrowed: u64 = response.get_u256(0).unwrap().try_into()?;
                let current_dept: u64 = response.get_u256(1).unwrap().try_into()?;
                let collateral_amount: u64 = response.get_u256(2).unwrap().try_into()?;
                let health_factor: u64 = response.get_u256(3).unwrap().try_into()?;
                let borrow_index: u64 = response.get_u256(4).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetUserBorrowPositionOutput {
                        principal_borrowed,
                        current_dept,
                        collateral_amount,
                        health_factor,
                        borrow_index,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetUserBorrowPosition(
                    output,
                ))
            }
            AssetLendingPoolFunctionsInput::GetMaxBorrowAmount(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                let mut params = ContractFunctionParameters::new();

                let collateral_amount = BigUint::from(args.collateral_amount);
                params.add_uint256(collateral_amount);
                params.add_address(args.collateral_asset.as_str());

                query_transaction.function_with_parameters("getMaxBorrowAmount", &params);

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let max_borrow_amount: u64 = response.get_u256(0).unwrap().try_into()?;
                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetMaxBorrowAmountOutput { max_borrow_amount }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetMaxBorrowAmount(output))
            }
            AssetLendingPoolFunctionsInput::IsPositionLiquidatable(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);
                let mut params = ContractFunctionParameters::new();

                params.add_address(args.user.as_str());
                params.add_address(args.collateral_asset.as_str());

                query_transaction.function_with_parameters("isPositionLiquidatable", &params);

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let liquidatable: bool = response.get_bool(0).unwrap();
                let health_factor: u64 = response.get_u256(1).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(IsPositionLiquidatableOutput {
                        liquidatable,
                        health_factor,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::IsPositionLiquidatable(
                    output,
                ))
            }
            AssetLendingPoolFunctionsInput::GetPoolStats(contract_id) => {
                query_transaction.contract_id(contract_id.parse()?);
                query_transaction.function("getPoolStats");

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let total_supplied: u64 = response.get_u256(0).unwrap().try_into()?;
                let total_borrowed: u64 = response.get_u256(1).unwrap().try_into()?;
                let liquidity: u64 = response.get_u256(2).unwrap().try_into()?;
                let utilization: u64 = response.get_u256(3).unwrap().try_into()?;
                let borrow_rate: u64 = response.get_u256(4).unwrap().try_into()?;
                let supply_rate: u64 = response.get_u256(5).unwrap().try_into()?;
                let borrow_index: u64 = response.get_u256(6).unwrap().try_into()?;
                let supply_index: u64 = response.get_u256(7).unwrap().try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetPoolStatsOutput {
                        total_supplied,
                        total_borrowed,
                        liquidity,
                        utilization,
                        borrow_rate,
                        supply_rate,
                        borrow_index,
                        supply_index,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetPoolStats(output))
            }
            AssetLendingPoolFunctionsInput::Deposit(args) => {
                transaction.contract_id(args.contract_id.parse()?);
                transaction.function("deposit");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.user.as_str());
                let amount = BigUint::from(args.amount);
                params.add_uint256(amount);

                transaction.function_parameters(params.to_bytes(Some("deposit")));
                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let record = response.get_record(&mut wallet.client).await?;

                let res = record
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Failed to get function result"))?;
                let supply_index: u64 = res
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Failed to get supply index"))?
                    .try_into()?;
                let yield_amount: u64 = res
                    .get_u256(1)
                    .ok_or_else(|| anyhow!("Failed to get supply index"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some((supply_index, yield_amount)),
                };

                Ok(AssetLendingPoolFunctionsOutput::Deposit(output))
            }
            AssetLendingPoolFunctionsInput::Withdraw(args) => {
                transaction.contract_id(args.contract_id.parse()?);
                transaction.function("withdraw");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.user.as_str());
                let yield_token_amount = BigUint::from(args.yield_token_amount);
                params.add_uint256(yield_token_amount);

                transaction.function_parameters(params.to_bytes(Some("withdraw")));
                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let record = response.get_record(&mut wallet.client).await?;

                let res = record
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Failed to get function result"))?;
                let supply_index: u64 = res
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Failed to get supply index"))?
                    .try_into()?;
                let underlying_value: u64 = res
                    .get_u256(1)
                    .ok_or_else(|| anyhow!("Failed to get supply index"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some((supply_index, underlying_value)),
                };

                Ok(AssetLendingPoolFunctionsOutput::Withdraw(output))
            }
            AssetLendingPoolFunctionsInput::Borrow(args) => {
                transaction.contract_id(args.contract_id.parse()?);
                transaction.function("borrow");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.user.as_str());
                let collateral_amount = BigUint::from(args.collateral_amount);
                params.add_uint256(collateral_amount);
                params.add_address(args.collateral_asset.as_str());

                transaction.function_parameters(params.to_bytes(Some("borrow")));
                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let record = response.get_record(&mut wallet.client).await?;
                let res = record
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Failed to get function result"))?;
                let borrow_index: u64 = res
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Failed to get borrow index"))?
                    .try_into()?;

                let borrowed_amount: u64 = res
                    .get_u256(1)
                    .ok_or_else(|| anyhow!("Failed to get borrowd amount"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some(BorrowResultArgs {
                        borrow_index,
                        borrowed_amount,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::Borrow(output))
            }
            AssetLendingPoolFunctionsInput::Repay(args) => {
                transaction.contract_id(args.contract_id.parse()?);
                transaction.function("repay");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.user.as_str());
                params.add_address(args.collateralized_asset.as_str());
                let repay_amount = BigUint::from(args.repay_amount);
                params.add_uint256(repay_amount);

                transaction.function_parameters(params.to_bytes(Some("repay")));
                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let record = response
                    .get_record(&wallet.client)
                    .await?
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Failed to get function result"))?;

                let collateral_unlocked: u64 = record
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Failed to get collateral amount"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some(RepayResultArgs {
                        collateral_unlocked,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::Repay(output))
            }
            AssetLendingPoolFunctionsInput::Liquidate(args) => {
                transaction.contract_id(args.contract_id.parse()?);
                transaction.function("liquidate");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.liquidator.as_str());
                params.add_address(args.borrower.as_str());
                let dept_to_cover = BigUint::from(args.dept_to_cover);
                params.add_uint256(dept_to_cover);
                params.add_address(args.collateral_asset.as_str());

                transaction.function_parameters(params.to_bytes(Some("liquidate")));
                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let record = response
                    .get_record(&wallet.client)
                    .await?
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Unable to get record result"))?;

                let collateral_amount_obtained: u64 = record
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Unable to obtain collateral amount"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some(LiquidateResultArgs {
                        obtained_collateral: collateral_amount_obtained,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::Liquidate(output))
            }
            AssetLendingPoolFunctionsInput::GetReserveAccount(contract_id) => {
                let params = ContractFunctionParameters::new();
                query_transaction.contract_id(contract_id.parse()?);
                query_transaction.function_with_parameters("getReserveAccount", &params);

                let response = query_transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let reserve_account = response.get_address(0).unwrap();

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetAccount {
                        account: reserve_account,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetReserveAccount(output))
            }
            AssetLendingPoolFunctionsInput::GetTreasuryAccount(contract_id) => {
                query_transaction.contract_id(contract_id.parse()?);
                let params = ContractFunctionParameters::new();
                query_transaction.function_with_parameters("getTreasuryAccount", &params);

                let response = query_transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let reserve_account = response.get_address(0).unwrap();

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(GetAccount {
                        account: reserve_account,
                    }),
                };

                Ok(AssetLendingPoolFunctionsOutput::GetReserveAccount(output))
            }
        }
    }
}
