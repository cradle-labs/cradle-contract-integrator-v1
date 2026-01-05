use crate::utils::functions::asset_lending::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters, Hbar};
use num_bigint::BigUint;
use std::convert::TryInto;
use tokio::time::Duration;

pub async fn get_utilization(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetUtilizationOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn get_borrow_rate(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetBorrowRateOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn get_supply_rate(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetSupplyRateOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn update_borrow_index(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

    transaction.contract_id(contract_id.parse()?);
    transaction.function("updateBorrowIndex");

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&mut wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn update_supply_index(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn update_indices(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn calculate_current_debt(
    args: CalculateCurrentDebtArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<CalculateCurrentDebtOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn calculate_current_deposit(
    args: CalculateCurrentDepositArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<CalculateCurrentDepositOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn calculate_health_factor(
    args: CalculateHealthFactorArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<CalculateHealthFactorOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn update_oracle(
    args: UpdateOracleArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn get_asset_multiplier(
    args: GetAssetMultiplierArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetAssetMultiplierOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn get_user_deposit_position(
    args: GetUserDepositPositon,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetUserDepositPositonOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn get_user_borrow_position(
    args: GetUserBorrowPosition,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetUserBorrowPositionOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

    query_transaction.contract_id(args.contract_id.parse()?);
    let mut params = ContractFunctionParameters::new();
    params.add_address(args.user.as_str());
    params.add_address(args.collateral_asset.as_str());
    query_transaction.function_with_parameters("getUserBorrowPosition", &params);

    let response = query_transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;
    let principal_borrowed: u64 = response.get_u256(0).unwrap().try_into()?;
    let current_dept: u64 = response.get_u256(1).unwrap().try_into()?;
    let collateral_amount: u64 = response.get_u256(2).unwrap().try_into()?;
    let health_factor: u64 = response.get_u256(3).unwrap().try_into().unwrap_or(1_u64);
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

    Ok(output)
}

pub async fn get_max_borrow_amount(
    args: GetMaxBorrowAmount,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetMaxBorrowAmountOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn is_position_liquidatable(
    args: IsPositionLiquidatableArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<IsPositionLiquidatableOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn get_pool_stats(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetPoolStatsOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn deposit(
    args: DepositArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<(u64, u64)>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn withdraw(
    args: WithdrawArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<(u64, u64)>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn borrow(
    args: BorrowArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<BorrowResultArgs>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn repay(
    args: RepayArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<RepayResultArgs>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn liquidate(
    args: LiquidateArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<LiquidateResultArgs>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}

pub async fn get_reserve_account(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetAccount>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}

pub async fn get_treasury_account(
    contract_id: String,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetAccount>> {
    let mut query_transaction = ContractCallQuery::new();
    query_transaction.gas(5_000_000);
    query_transaction.payment_amount(Hbar::new(10));

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

    Ok(output)
}
