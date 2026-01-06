use crate::utils::functions::native_asset_issuer_factory::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, ContractId};
use std::str::FromStr;
use tokio::time::Duration;

pub async fn create_native_asset_issuer(
    args: CreateNativeAssetIssuerArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<CreateNativeAssetIssuerResult>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);

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

    Ok(output)
}
