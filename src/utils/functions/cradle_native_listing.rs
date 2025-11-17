use anyhow::anyhow;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::utils::functions::{
    FunctionCallOutput, WithContractId, commons::ContractFunctionProcessor,
};
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ListingStatus {
    Pending,
    Open,
    Closed,
    Paused,
    Cancelled,
}

impl From<u8> for ListingStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Pending,
            1 => Self::Open,
            2 => Self::Closed,
            3 => Self::Paused,
            4 => Self::Cancelled,
            _ => Self::Cancelled,
        }
    }
}

impl ListingStatus {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Pending => 0,
            Self::Open => 1,
            Self::Closed => 2,
            Self::Paused => 3,
            Self::Cancelled => 5,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PurchaseInputArgs {
    pub buyer: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReturnAssetInputArgs {
    pub account: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawToBeneficiaryInputArgs {
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CradleNativeListingFunctionsInput {
    UpdateListingStatus(WithContractId<ListingStatus>),
    Purchase(WithContractId<PurchaseInputArgs>),
    ReturnAsset(WithContractId<ReturnAssetInputArgs>),
    WithdrawToBeneficiary(WithContractId<WithdrawToBeneficiaryInputArgs>),
    GetListingStats(WithContractId<()>),
    GetFee(WithContractId<u64>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListingStats {
    pub total_distributed: u64,
    pub remaining: u64,
    pub raised: u64,
    pub balance: u64,
    pub status: ListingStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CradleNativeListingFunctionsOutput {
    UpdateListingStatus(FunctionCallOutput<()>),
    Purchase(FunctionCallOutput<u64>),
    ReturnAsset(FunctionCallOutput<u64>),
    WithdrawToBeneficiary(FunctionCallOutput<()>),
    GetListingStats(FunctionCallOutput<ListingStats>),
    GetFee(FunctionCallOutput<u64>),
}

impl ContractFunctionProcessor<CradleNativeListingFunctionsOutput>
    for CradleNativeListingFunctionsInput
{
    async fn process(
        &self,
        wallet: &mut crate::wallet::wallet::ActionWallet,
    ) -> anyhow::Result<CradleNativeListingFunctionsOutput> {
        let mut transaction = ContractExecuteTransaction::new();

        let mut query_transaction = ContractCallQuery::new();

        query_transaction.gas(10_000_000);

        transaction.gas(10_000_000);

        let mut params = ContractFunctionParameters::new();

        match self {
            CradleNativeListingFunctionsInput::UpdateListingStatus(args) => {
                transaction.contract_id(args.contract_id.parse()?);

                let param_args = args
                    .rest
                    .as_ref()
                    .ok_or_else(|| anyhow!("Unable to extract params"))?;

                params.add_uint8(param_args.to_u8());

                transaction.function_with_parameters("updateListingStatus", &params);

                let response = transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let output = FunctionCallOutput {
                    transaction_id: response.transaction_id.to_string(),
                    output: None,
                };

                Ok(CradleNativeListingFunctionsOutput::UpdateListingStatus(
                    output,
                ))
            }
            CradleNativeListingFunctionsInput::Purchase(args) => {
                transaction.contract_id(args.contract_id.parse()?);

                let param_args = args
                    .rest
                    .as_ref()
                    .ok_or_else(|| anyhow!("Unable to extract params"))?;

                params.add_address(&param_args.buyer);
                let valid_amount = BigUint::from(param_args.amount);
                params.add_uint256(valid_amount);

                transaction.function_with_parameters("purchase", &params);

                let response = transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let record = response
                    .get_record(&wallet.client)
                    .await?
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Unable to retrieve result"))?;

                let received_listing_assets: u64 = record
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Unable to receive listing"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: response.transaction_id.to_string(),
                    output: Some(received_listing_assets),
                };

                Ok(CradleNativeListingFunctionsOutput::Purchase(output))
            }
            CradleNativeListingFunctionsInput::ReturnAsset(args) => {
                transaction.contract_id(args.contract_id.parse()?);

                let param_args = args
                    .rest
                    .as_ref()
                    .ok_or_else(|| anyhow!("unable to extract params"))?;

                params.add_address(&param_args.account);
                let valid_amount = BigUint::from(param_args.amount);
                params.add_uint256(valid_amount);

                transaction.function_with_parameters("returnAsset", &params);

                let response = transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let record = response
                    .get_record(&wallet.client)
                    .await?
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Unable to retrieve result"))?;

                let received_purchase_asset: u64 = record
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Unable to extract received amount"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: response.transaction_id.to_string(),
                    output: Some(received_purchase_asset),
                };

                Ok(CradleNativeListingFunctionsOutput::ReturnAsset(output))
            }
            CradleNativeListingFunctionsInput::WithdrawToBeneficiary(args) => {
                transaction.contract_id(args.contract_id.parse()?);

                let param_args = args
                    .rest
                    .as_ref()
                    .ok_or_else(|| anyhow!("Param extraction failed"))?;

                let valid_amount = BigUint::from(param_args.amount);

                params.add_uint256(valid_amount);

                transaction.function_with_parameters("withdrawToBeneficiary", &params);

                let response = transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let output = FunctionCallOutput {
                    transaction_id: response.transaction_id.to_string(),
                    output: None,
                };

                Ok(CradleNativeListingFunctionsOutput::WithdrawToBeneficiary(
                    output,
                ))
            }
            CradleNativeListingFunctionsInput::GetListingStats(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);

                query_transaction.function("getListingStats");

                let response = query_transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let total_distributed: u64 = response
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("Unable to extract result"))?
                    .try_into()?;
                let remaining: u64 = response
                    .get_u256(1)
                    .ok_or_else(|| anyhow!("Unable to extract result"))?
                    .try_into()?;
                let raised: u64 = response
                    .get_u256(2)
                    .ok_or_else(|| anyhow!("Unable to extract result"))?
                    .try_into()?;
                let balance: u64 = response
                    .get_u256(3)
                    .ok_or_else(|| anyhow!("Unable to extract result"))?
                    .try_into()?;
                let status: u8 = response
                    .get_u8(4)
                    .ok_or_else(|| anyhow!("unable to extract result"))?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(ListingStats {
                        total_distributed,
                        remaining,
                        raised,
                        balance,
                        status: ListingStatus::from(status),
                    }),
                };

                Ok(CradleNativeListingFunctionsOutput::GetListingStats(output))
            }
            CradleNativeListingFunctionsInput::GetFee(args) => {
                query_transaction.contract_id(args.contract_id.parse()?);

                let value = args
                    .rest
                    .as_ref()
                    .ok_or_else(|| anyhow!("unable to extract amount"))?;

                let amount = BigUint::from(*value);

                params.add_uint256(amount);

                query_transaction.function_with_parameters("getFee", &params);

                let response = query_transaction
                    .execute_with_timeout(&wallet.client, Duration::from_secs(180))
                    .await?;

                let fee_applied: u64 = response
                    .get_u256(0)
                    .ok_or_else(|| anyhow!("unable to extract fee"))?
                    .try_into()?;

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(fee_applied),
                };

                Ok(CradleNativeListingFunctionsOutput::GetFee(output))
            }
        }
    }
}
