use crate::utils::functions::FunctionCallOutput;
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, ContractId};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateNativeAssetIssuerArgs {
    pub treasury: String,
    pub acl_contract: String,
    pub allow_list: u64,
    pub reserve_token: String,
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NativeAssetIssuerFactoryFunctionsInput {
    CreateNativeAssetIssuer(CreateNativeAssetIssuerArgs),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateNativeAssetIssuerResult {
    pub issuer_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NativeAssetIssuerFactoryFunctionsOutput {
    CreateNativeAssetIssuer(FunctionCallOutput<CreateNativeAssetIssuerResult>),
}

impl ContractFunctionProcessor<NativeAssetIssuerFactoryFunctionsOutput>
    for NativeAssetIssuerFactoryFunctionsInput
{
    async fn process(
        &self,
        wallet: &mut ActionWallet,
    ) -> anyhow::Result<NativeAssetIssuerFactoryFunctionsOutput> {
        let mut transaction = ContractExecuteTransaction::new();
        transaction.gas(10_000_000);

        match self {
            NativeAssetIssuerFactoryFunctionsInput::CreateNativeAssetIssuer(args) => {
                let contract_id = ContractId::from_str(args.contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("createNativeAssetIssuer");

                let mut params = ContractFunctionParameters::new();
                params.add_address(args.treasury.as_str());
                params.add_address(args.acl_contract.as_str());
                params.add_uint64(args.allow_list);
                params.add_address(args.reserve_token.as_str());

                transaction.function_parameters(params.to_bytes(Some("createNativeAssetIssuer")));

                let response = transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;
                let record = response.get_record(&mut wallet.client).await?;

                let result = record
                    .contract_function_result
                    .ok_or_else(|| anyhow!("Failed to find contract result"))?;

                let issuer_address = result
                    .get_address(0)
                    .ok_or_else(|| anyhow!("Failed to find issuer address"))?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some(CreateNativeAssetIssuerResult { issuer_address }),
                };

                Ok(
                    NativeAssetIssuerFactoryFunctionsOutput::CreateNativeAssetIssuer(output),
                )
            }
        }
    }
}
