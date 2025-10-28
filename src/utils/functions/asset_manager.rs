use std::str::FromStr;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, ContractId, TokenAssociateTransaction, TokenId};
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use crate::utils::functions::commons::ContractFunctionProcessor;
use tokio::time::Duration;

pub struct MintArgs {
    pub asset_contract: String,
    pub amount: u64,
}

pub struct BurnArgs {
    pub asset_contract: String,
    pub amount: u64,
}

pub struct WipeArgs {
    pub asset_contract: String,
    pub account: String,
    pub amount: u64,
}

pub struct AirdropArgs {
    pub asset_contract: String,
    pub target: String,
    pub amount: u64,
}

pub struct TransferArgs {
    pub asset_contract: String,
    pub target: String,
    pub amount: u64
}

pub enum AssetManagerFunctionInput {
    Mint(MintArgs),
    Burn(BurnArgs),
    Wipe(WipeArgs),
    Airdrop(AirdropArgs),
    SelfAssociate,
    GrantKYC(String, String),
    Transfer(TransferArgs),
    TokenAssociate(String),
}





pub enum AssetManagerFunctionOutput {
    Mint(FunctionCallOutput<()>),
    Burn(FunctionCallOutput<()>),
    Wipe(FunctionCallOutput<()>),
    Airdrop(FunctionCallOutput<()>),
    SelfAssociate(FunctionCallOutput<()>),
    GrantKYC(FunctionCallOutput<()>),
    Transfer(FunctionCallOutput<()>),
    TokenAssociate(FunctionCallOutput<()>),
}


impl ContractFunctionProcessor<AssetManagerFunctionOutput> for AssetManagerFunctionInput {
    async fn process(&self, wallet: &mut ActionWallet) -> anyhow::Result<AssetManagerFunctionOutput> {
        let mut transaction = ContractExecuteTransaction::new();
        let mut params = ContractFunctionParameters::new();
        transaction.gas(5_000_000);
        match self {
            AssetManagerFunctionInput::Mint(args) => {
                transaction.contract_id(args.asset_contract.parse::<ContractId>()?);
                transaction.function("mint");

                params.add_uint64(args.amount);
                transaction.function_parameters(params.to_bytes(Some("mint")));

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::Mint(output))

            }
            AssetManagerFunctionInput::Burn(args) => {
                transaction.contract_id(args.asset_contract.parse()?);
                transaction.function("burn");

                params.add_uint64(args.amount);
                transaction.function_parameters(params.to_bytes(Some("burn")));

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::Burn(output))
            }
            AssetManagerFunctionInput::Wipe(args) => {
                transaction.contract_id(args.asset_contract.parse()?);
                transaction.function("wipe");

                params.add_uint64(args.amount);
                params.add_address(&args.account);
                transaction.function_parameters(params.to_bytes(Some("wipe")));

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::Wipe(output))
            }
            AssetManagerFunctionInput::Airdrop(args) => {
                transaction.contract_id(args.asset_contract.parse()?);
                transaction.function("airdropTokens");

                params.add_address(&args.target);
                params.add_uint64(args.amount);
                transaction.function_parameters(params.to_bytes(Some("airdropTokens")));

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::Airdrop(output))
            }
            AssetManagerFunctionInput::SelfAssociate => {


                transaction.contract_id(wallet.account_id.parse()?);
                transaction.function("selfAssociate");

                transaction.function_parameters(params.to_bytes(Some("selfAssociate")));

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::SelfAssociate(output))
            }

            AssetManagerFunctionInput::GrantKYC(asset_manager, target)=>{
                let contract_id = ContractId::from_str(asset_manager.as_str())?;
                transaction.contract_id(contract_id);

                params.add_address(target.as_str());
                transaction.function_with_parameters("grantKyc", &params);

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::GrantKYC(output))
            },
            AssetManagerFunctionInput::Transfer(args) => {
                transaction.contract_id(args.asset_contract.parse()?);
                transaction.function("transferTokens");

                params.add_address(&args.target);
                params.add_uint64(args.amount);
                transaction.function_parameters(params.to_bytes(Some("transferTokens")));

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::Transfer(output))
            },
            AssetManagerFunctionInput::TokenAssociate(id)  => {
                let mut transaction = TokenAssociateTransaction::new();

                transaction.account_id(wallet.account_id.parse()?);
                transaction.token_ids(vec![
                    id.as_str().parse()?
                ]);

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;
                let receipt = response.get_receipt(&wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None
                };

                Ok(AssetManagerFunctionOutput::TokenAssociate(output))

            }
        }
    }
}
