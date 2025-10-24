use std::str::FromStr;
use std::time::Duration;
use crate::utils::functions::FunctionCallOutput;
use super::commons::ContractFunctionProcessor;
use crate::wallet::wallet::ActionWallet;
use anyhow::Result;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters, ContractId};
use num_bigint::BigUint;

pub struct AssociateTokenArgs {
    pub token: String,
    pub account_contract_id: String
}

pub struct DepositArgs {
    pub token: String,
    pub amount: u64,
    pub account_contract_id: String
}

pub struct WithdrawArgs {
    pub asset: String,
    pub amount: u64,
    pub to: String,
    pub account_contract_id: String
}

pub struct UpdateBridgingStatusArgs {
    pub new_status: bool,
    pub account_contract_id: String
}

pub struct TransferAssetArgs {
    pub asset: String,
    pub amount: u64,
    pub to: String,
    pub account_contract_id: String
}

pub struct GetTradableBalanceArgs {
    pub asset: String,
    pub account_contract_id: String
}

pub struct LockAssetArgs {
    pub asset: String,
    pub amount: u64,
    pub account_contract_id: String
}

pub struct UnLockAssetArgs {
    pub asset: String,
    pub amount: u64,
    pub account_contract_id: String
}

pub struct AddLoanLockArgs {
    pub lender: String,
    pub collateral: String,
    pub loan_amount: u64,
    pub collateral_amount: u64,
    pub borrow_index: u64,
    pub account_contract_id: String
}

pub struct GetLoanAmountArgs {
    pub lender: String,
    pub collateral: String,
    pub account_contract_id: String
}

pub struct GetCollateralArgs {
    pub lender: String,
    pub collateral: String,
    pub account_contract_id: String
}

pub struct GetLoanBlockIndexArgs {
    pub lender: String,
    pub collateral: String,
    pub account_contract_id: String
}

pub struct RemoveLoanLockArgs {
    pub lender: String,
    pub collateral: String,
    pub loan_amount: u64,
    pub collateral_amount: u64,
    pub borrow_index: u64,
    pub account_contract_id: String
}


pub enum CradleAccountFunctionInput {
    AssociateToken(AssociateTokenArgs),
    Deposit(DepositArgs),
    Withdraw(WithdrawArgs),
    UpdateBridgingStatus(UpdateBridgingStatusArgs),
    TransferAsset(TransferAssetArgs),
    GetTradableBalance(GetTradableBalanceArgs),
    LockAsset(LockAssetArgs),
    UnLockAsset(UnLockAssetArgs),
    AddLoanLock(AddLoanLockArgs),
    GetLoanAmount(GetLoanAmountArgs),
    GetCollateral(GetCollateralArgs),
    GetLoanBlockIndex(GetLoanBlockIndexArgs),
    RemoveLoanLock(RemoveLoanLockArgs)
}

pub struct GetLoanAmountOutput {
    pub loan_amount: u64
}

pub struct GetCollateralOutput {
    pub collateral_amount: u64
}

pub struct GetLoanBlockIndexOutput {
    pub block_index: u64
}

pub struct GetTradableBalanceOutput {
    pub tradable_balance: u64
}

pub enum CradleAccountFunctionOutput {
    AssociateToken(FunctionCallOutput<()>),
    Deposit(FunctionCallOutput<()>),
    Withdraw(FunctionCallOutput<()>),
    UpdateBridgingStatus(FunctionCallOutput<()>),
    TransferAsset(FunctionCallOutput<()>),
    GetTradableBalance(FunctionCallOutput<GetTradableBalanceOutput>),
    LockAsset(FunctionCallOutput<()>),
    UnLockAsset(FunctionCallOutput<()>),
    AddLoanLock(FunctionCallOutput<()>),
    GetLoanAmount(FunctionCallOutput<GetLoanAmountOutput>),
    GetCollateral(FunctionCallOutput<GetCollateralOutput>),
    GetLoanBlockIndex(FunctionCallOutput<GetLoanBlockIndexOutput>),
    RemoveLoanLock(FunctionCallOutput<()>)
}

impl ContractFunctionProcessor<CradleAccountFunctionOutput> for CradleAccountFunctionInput {
    
    async fn process(&self, wallet: &mut ActionWallet) -> Result<CradleAccountFunctionOutput> {

        let mut transaction = ContractExecuteTransaction::new();
        transaction.gas(5_000_000);
        let mut query_transaction = ContractCallQuery::new();
        let mut params = ContractFunctionParameters::new();

        match self {
            CradleAccountFunctionInput::AssociateToken(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("associateToken");

                params.add_address(args.token.as_str());

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(CradleAccountFunctionOutput::AssociateToken(output))
            },
            CradleAccountFunctionInput::Deposit(_args)=>{
                unimplemented!("This is only meant to be called on the frontend")
            }
            CradleAccountFunctionInput::Withdraw(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("withdraw");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.asset.as_str());
                params.add_uint64(args.amount);
                params.add_address(args.to.as_str());

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(CradleAccountFunctionOutput::Withdraw(output))

            }
            CradleAccountFunctionInput::UpdateBridgingStatus(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("updateBridgingStatus");

                let mut params = ContractFunctionParameters::new();

                params.add_bool(args.new_status);

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(CradleAccountFunctionOutput::UpdateBridgingStatus(output))
            }
            CradleAccountFunctionInput::TransferAsset(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("transferAsset");
                let mut params = ContractFunctionParameters::new();

                params.add_address(args.to.as_str());
                params.add_address(args.asset.as_str());
                let amount = BigUint::from(args.amount);
                params.add_uint256(amount);
                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };
                Ok(CradleAccountFunctionOutput::TransferAsset(output))
            }
            CradleAccountFunctionInput::GetTradableBalance(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                query_transaction.contract_id(contract_id);

                let mut params = ContractFunctionParameters::new();
                params.add_address(args.asset.as_str());

                let response = query_transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let tradable_balance: u64 = response.get_u256(0).unwrap().try_into()?;
                let output = FunctionCallOutput{
                    transaction_id: "".to_string(),
                    output: Some(GetTradableBalanceOutput {
                        tradable_balance
                    })
                };


                Ok(CradleAccountFunctionOutput::GetTradableBalance(output))
            }
            CradleAccountFunctionInput::LockAsset(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("lockAsset");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.asset.as_str());
                let amount = BigUint::from(args.amount);
                params.add_uint256(amount);

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(CradleAccountFunctionOutput::LockAsset(output))
            }
            CradleAccountFunctionInput::UnLockAsset(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("unlockAsset");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.asset.as_str());
                let amount = BigUint::from(args.amount);
                params.add_uint256(amount);

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(CradleAccountFunctionOutput::UnLockAsset(output))
            }
            CradleAccountFunctionInput::AddLoanLock(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("addLoanLock");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.lender.as_str());
                params.add_address(args.collateral.as_str());
                let loan_amount = BigUint::from(args.loan_amount);
                params.add_uint256(loan_amount);
                let collateral_amount = BigUint::from(args.collateral_amount);
                params.add_uint256(collateral_amount);
                let borrow_index = BigUint::from(args.borrow_index);
                params.add_uint256(borrow_index);

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(CradleAccountFunctionOutput::AddLoanLock(output))
            }
            CradleAccountFunctionInput::GetLoanAmount(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                query_transaction.contract_id(contract_id);
                query_transaction.function("getLoanAmount");

                let mut params = ContractFunctionParameters::new();
                params.add_address(args.lender.as_str());
                params.add_address(args.collateral.as_str());

                let response = query_transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let loan_amount: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput{
                    transaction_id: "".to_string(),
                    output: Some(GetLoanAmountOutput {
                        loan_amount
                    })
                };

                Ok(CradleAccountFunctionOutput::GetLoanAmount(output))
            }
            CradleAccountFunctionInput::GetCollateral(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                query_transaction.contract_id(contract_id);

                let mut params = ContractFunctionParameters::new();
                params.add_address(args.lender.as_str());
                params.add_address(args.collateral.as_str());

                let response = query_transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let collateral_amount: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput{
                    transaction_id: "".to_string(),
                    output: Some(GetCollateralOutput {
                        collateral_amount
                    })
                };

                Ok(CradleAccountFunctionOutput::GetCollateral(output))
            }
            CradleAccountFunctionInput::GetLoanBlockIndex(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                query_transaction.contract_id(contract_id);

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.lender.as_str());
                params.add_address(args.collateral.as_str());

                let response = query_transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;
                let block_index: u64 = response.get_u256(0).unwrap().try_into()?;

                let output = FunctionCallOutput{
                    transaction_id: "".to_string(),
                    output: Some(GetLoanBlockIndexOutput {
                        block_index
                    })
                };

                Ok(CradleAccountFunctionOutput::GetLoanBlockIndex(output))
            }
            CradleAccountFunctionInput::RemoveLoanLock(args)=>{
                let contract_id = ContractId::from_str(args.account_contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("removeLoanLock");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.lender.as_str());
                params.add_address(args.collateral.as_str());

                let loan_amount = BigUint::from(args.loan_amount);
                let collateral_amount = BigUint::from(args.collateral_amount);
                let borrow_index = BigUint::from(args.borrow_index);

                params.add_uint256(loan_amount);
                params.add_uint256(collateral_amount);
                params.add_uint256(borrow_index);

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput{
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(CradleAccountFunctionOutput::RemoveLoanLock(output))
            }
        }
    }
    
}