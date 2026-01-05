use crate::utils::functions::cradle_account::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{
    ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters, ContractId,
};
use num_bigint::BigUint;
use std::str::FromStr;
use tokio::time::Duration;

pub async fn associate_token(
    args: AssociateTokenArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);
    params.add_address(args.token.as_str());
    transaction.function_with_parameters("associateToken", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn deposit(
    _args: DepositArgs,
    _wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    unimplemented!("This is only meant to be called on the frontend")
}

pub async fn withdraw(
    args: WithdrawArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);

    params.add_address(args.asset.as_str());
    params.add_uint64(args.amount);
    params.add_address(args.to.as_str());

    transaction.function_with_parameters("withdraw", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn update_bridging_status(
    args: UpdateBridgingStatusArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);

    params.add_bool(args.new_status);
    transaction.function_with_parameters("updateBridgingStatus", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn transfer_asset(
    args: TransferAssetArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);

    params.add_address(args.to.as_str());
    params.add_address(args.asset.as_str());
    let amount = BigUint::from(args.amount);
    params.add_uint256(amount);
    transaction.function_with_parameters("transferAsset", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };
    Ok(output)
}

pub async fn get_tradable_balance(
    args: GetTradableBalanceArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetTradableBalanceOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    query_transaction.contract_id(contract_id);

    let mut params = ContractFunctionParameters::new();
    params.add_address(args.asset.as_str());

    query_transaction.function_with_parameters("getTradableBalance", &params);

    let response = query_transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let tradable_balance: u64 = response.get_u256(0).unwrap().try_into()?;
    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(GetTradableBalanceOutput { tradable_balance }),
    };

    Ok(output)
}

pub async fn lock_asset(
    args: LockAssetArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);

    params.add_address(args.asset.as_str());
    let amount = BigUint::from(args.amount);
    params.add_uint256(amount);
    transaction.function_with_parameters("lockAsset", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn unlock_asset(
    args: UnLockAssetArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);

    params.add_address(args.asset.as_str());
    let amount = BigUint::from(args.amount);
    params.add_uint256(amount);
    transaction.function_with_parameters("unlockAsset", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;
    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn add_loan_lock(
    args: AddLoanLockArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);

    params.add_address(args.lender.as_str());
    params.add_address(args.collateral.as_str());
    let loan_amount = BigUint::from(args.loan_amount);
    params.add_uint256(loan_amount);
    let collateral_amount = BigUint::from(args.collateral_amount);
    params.add_uint256(collateral_amount);
    let borrow_index = BigUint::from(args.borrow_index);
    params.add_uint256(borrow_index);
    transaction.function_with_parameters("addLoanLock", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}

pub async fn get_loan_amount(
    args: GetLoanAmountArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetLoanAmountOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    query_transaction.contract_id(contract_id);

    let mut params = ContractFunctionParameters::new();
    params.add_address(args.lender.as_str());
    params.add_address(args.collateral.as_str());
    query_transaction.function_with_parameters("getLoanAmount", &params);

    let response = query_transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let loan_amount: u64 = response.get_u256(0).unwrap().try_into()?;

    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(GetLoanAmountOutput { loan_amount }),
    };

    Ok(output)
}

pub async fn get_collateral(
    args: GetCollateralArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetCollateralOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    query_transaction.contract_id(contract_id);

    let mut params = ContractFunctionParameters::new();
    params.add_address(args.lender.as_str());
    params.add_address(args.collateral.as_str());

    query_transaction.function_with_parameters("getCollateral", &params);

    let response = query_transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let collateral_amount: u64 = response.get_u256(0).unwrap().try_into()?;

    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(GetCollateralOutput { collateral_amount }),
    };

    Ok(output)
}

pub async fn get_loan_block_index(
    args: GetLoanBlockIndexArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<GetLoanBlockIndexOutput>> {
    let mut query_transaction = ContractCallQuery::new();
    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    query_transaction.contract_id(contract_id);

    let mut params = ContractFunctionParameters::new();

    params.add_address(args.lender.as_str());
    params.add_address(args.collateral.as_str());

    query_transaction.function_with_parameters("getLoanBlockIndex", &params);

    let response = query_transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;
    let block_index: u64 = response.get_u256(0).unwrap().try_into()?;

    let output = FunctionCallOutput {
        transaction_id: "".to_string(),
        output: Some(GetLoanBlockIndexOutput { block_index }),
    };

    Ok(output)
}

pub async fn remove_loan_lock(
    args: RemoveLoanLockArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<()>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
    transaction.contract_id(contract_id);

    params.add_address(args.lender.as_str());
    params.add_address(args.collateral.as_str());

    let loan_amount = BigUint::from(args.loan_amount);
    let collateral_amount = BigUint::from(args.collateral_amount);
    let borrow_index = BigUint::from(args.borrow_index);

    params.add_uint256(loan_amount);
    params.add_uint256(collateral_amount);
    params.add_uint256(borrow_index);
    transaction.function_with_parameters("removeLoanLock", &params);

    let response = transaction
        .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: None,
    };

    Ok(output)
}
