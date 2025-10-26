use std::str::FromStr;
use std::time::Duration;
use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, ContractId};
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;

pub struct CreateAssetArgs {
    pub name: String,
    pub symbol: String,
    pub acl_contract: String,
    pub allow_list: u64,
    pub contract_id: String
}


pub struct LockReservesArgs {
    pub user: String,
    pub amount: u64,
    pub contract_id: String
}

pub struct ReleaseAssetArgs {
    pub user: String,
    pub symbol: String,
    pub mint_amount: u64,
    pub unlock_amount: u64,
    pub contract_id: String
}

pub struct LockAssetArgs {
    pub user: String,
    pub asset: String,
    pub amount: u64,
    pub contract_id: String
}

pub struct ReleaseReservesArgs {
    pub user: String,
    pub symbol:String,
    pub burn_amount: u64,
    pub release_amount: u64,
    pub contract_id: String
}

pub enum AssetIssuerFunctionsInput {
    CreateAsset(CreateAssetArgs),
    LockReserves(LockReservesArgs),
    ReleaseAsset(ReleaseAssetArgs),
    LockAsset(LockAssetArgs),
    ReleaseReserves(ReleaseReservesArgs)
}


pub struct CreateAssetResult {
    pub asset_manager: String,
    pub token: String
}


pub enum AssetIssuerFunctionsOutput {
    CreateAsset(FunctionCallOutput<CreateAssetResult>),
    LockReserves(FunctionCallOutput<()>),
    ReleaseAsset(FunctionCallOutput<()>),
    LockAsset(FunctionCallOutput<()>),
    ReleaseReserves(FunctionCallOutput<()>)
}


impl ContractFunctionProcessor<AssetIssuerFunctionsOutput> for AssetIssuerFunctionsInput {
    async fn process(&self, wallet: &mut ActionWallet) -> anyhow::Result<AssetIssuerFunctionsOutput> {
        let mut transaction = ContractExecuteTransaction::new();
        transaction.gas(5_000_000);
        match self {
            AssetIssuerFunctionsInput::CreateAsset(args)=>{
                let contract_id = ContractId::from_str(args.contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("createAsset");

                let mut params = ContractFunctionParameters::new();
                params.add_string(args.name.as_str());
                params.add_string(args.symbol.as_str());
                params.add_string(args.acl_contract.as_str());
                params.add_uint64(args.allow_list);

                transaction.function_parameters(params.to_bytes(Some("createAsset")));

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let record = response.get_record(&mut wallet.client).await?;
                let result = record.contract_function_result.ok_or_else(|| anyhow!("Failed to find contract result"))?;
                let asset_manager = result.get_address(0).ok_or_else(|| anyhow!("Failed to find asset manager"))?;
                let token_address = result.get_address(1).ok_or_else(|| anyhow!("Failed to find token address"))?;



                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some(CreateAssetResult {
                        token: token_address,
                        asset_manager
                    })
                };

                Ok(AssetIssuerFunctionsOutput::CreateAsset(output))
            },
            AssetIssuerFunctionsInput::LockReserves(args)=>{
                let contract_id = ContractId::from_str(args.contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("lockReserves");

                let mut params = ContractFunctionParameters::new();
                let amount = num_bigint::BigUint::from(args.amount);
                params.add_address(args.user.as_str());
                params.add_uint256(amount);

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetIssuerFunctionsOutput::LockReserves(output))
            },
            AssetIssuerFunctionsInput::ReleaseAsset(args)=>{

                let contract_id = ContractId::from_str(args.contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("releaseAsset");

                let mut params = ContractFunctionParameters::new();
                params.add_address(args.user.as_str());
                params.add_string(args.symbol.as_str());
                let mint_amount = num_bigint::BigUint::from(args.mint_amount);
                params.add_uint256(mint_amount);
                let unlock_amount = num_bigint::BigUint::from(args.unlock_amount);
                params.add_uint256(unlock_amount);
                transaction.function_parameters(params.to_bytes(Some("releaseAsset")));

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetIssuerFunctionsOutput::ReleaseAsset(output))
            },
            AssetIssuerFunctionsInput::LockAsset(args)=>{
                let contract_id = ContractId::from_str(args.contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("releaseAsset");

                let mut params = ContractFunctionParameters::new();
                params.add_address(args.user.as_str());
                params.add_address(args.asset.as_str());
                let amount = num_bigint::BigUint::from(args.amount);
                params.add_uint256(amount);
                transaction.function_parameters(params.to_bytes(Some("lockAsset")));

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };
                Ok(AssetIssuerFunctionsOutput::LockAsset(output))
            },
            AssetIssuerFunctionsInput::ReleaseReserves(args)=>{
                let contract_id = ContractId::from_str(args.contract_id.as_str())?;
                transaction.contract_id(contract_id);
                transaction.function("releaseAsset");

                let mut params = ContractFunctionParameters::new();

                params.add_address(args.user.as_str());
                params.add_string(args.symbol.as_str());
                let burn_amount = num_bigint::BigUint::from(args.burn_amount);
                params.add_uint256(burn_amount);
                let release_amount = num_bigint::BigUint::from(args.release_amount);
                params.add_uint256(release_amount);
                transaction.function_parameters(params.to_bytes(Some("releaseReserves")));

                let response = transaction.execute_with_timeout(&mut wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetIssuerFunctionsOutput::ReleaseAsset(output))
            }
        }
    }
}