use crate::utils::functions::asset_factory::*;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, Hbar};
use tokio::time::Duration;

pub async fn create_asset(
    args: CreateAssetArgs,
    wallet: &mut ActionWallet,
) -> anyhow::Result<FunctionCallOutput<CreateAssetOutput>> {
    let mut transaction = ContractExecuteTransaction::new();
    transaction.gas(10_000_000);
    let mut params = ContractFunctionParameters::new();

    let contract_ids = wallet.get_contract_ids()?;
    transaction.contract_id(contract_ids.asset_factory);
    transaction.max_transaction_fee(Hbar::new(60));
    transaction.function("createAsset");
    transaction.payable_amount(Hbar::new(50));
    params.add_string(&args.name);
    params.add_string(&args.symbol);

    transaction.function_parameters(params.to_bytes(Some("createAsset")));

    let response = transaction
        .execute_with_timeout(&wallet.client, Duration::from_secs(180))
        .await?;

    let receipt = response.get_receipt(&wallet.client).await?;

    let record = response.get_record(&wallet.client).await?;

    let returned = record
        .contract_function_result
        .ok_or_else(|| anyhow!("Failed to get function result"))?;

    let asset_manager_address = returned
        .get_address(0)
        .ok_or_else(|| anyhow!("Failed to get asset manager address"))?;
    let token_address = returned
        .get_address(1)
        .ok_or_else(|| anyhow!("Failed to get token address"))?;

    let output = FunctionCallOutput {
        transaction_id: receipt.transaction_id.unwrap().to_string(),
        output: Some(CreateAssetOutput {
            asset_manager: asset_manager_address,
            token: token_address,
        }),
    };

    Ok(output)
}
