use anyhow::anyhow;
use hedera::{ContractExecuteTransaction, ContractFunctionParameters, Hbar};
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::utils::functions::FunctionCallOutput;
use crate::wallet::wallet::ActionWallet;
use tokio::time::Duration;

pub struct CreateAssetArgs {
    pub name: String,
    pub symbol: String,
    pub acl_contract: String,
    pub allow_list: u64
}

pub enum AssetFactoryFunctionInput {
    CreateAsset(CreateAssetArgs)
}

pub struct CreateAssetOutput {
    pub asset_manager: String,
    pub token: String
}

pub enum AssetFactoryFunctionOutput {
    CreateAsset(FunctionCallOutput<CreateAssetOutput>)
}


impl ContractFunctionProcessor<AssetFactoryFunctionOutput> for AssetFactoryFunctionInput {
    async fn process(&self, wallet: &mut ActionWallet) -> anyhow::Result<AssetFactoryFunctionOutput> {

        let mut transaction= ContractExecuteTransaction::new();
        transaction.gas(10_000_000);
        let mut params = ContractFunctionParameters::new();

        match self {
            AssetFactoryFunctionInput::CreateAsset(args)=>{
                let contract_ids = wallet.get_contract_ids()?;
                transaction.contract_id(contract_ids.asset_factory);
                transaction.max_transaction_fee(Hbar::new(60));
                transaction.function("createAsset");
                transaction.payable_amount(Hbar::new(50));
                params.add_string(&args.name);
                params.add_string(&args.symbol);
                params.add_address(&args.acl_contract);
                params.add_uint64(args.allow_list);

                transaction.function_parameters(params.to_bytes(Some("createAsset")));

                let response = transaction.execute_with_timeout(&wallet.client, Duration::from_secs(180)).await?;

                let receipt = response.get_receipt(&wallet.client).await?;

                let record = response.get_record(&wallet.client).await?;

                let returned = record.contract_function_result.ok_or_else(||anyhow!("Failed to get function result"))?;

                let asset_manager_address = returned.get_address(0).ok_or_else(||anyhow!("Failed to get asset manager address"))?;
                let token_address = returned.get_address(1).ok_or_else(||anyhow!("Failed to get token address"))?;


                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: Some(CreateAssetOutput {
                        asset_manager: asset_manager_address,
                        token: token_address
                    })
                };


                Ok(AssetFactoryFunctionOutput::CreateAsset(output))
            }
        }
    }
}


#[cfg(test)]
mod asset_factory_tests {
    use std::env;
    use super::*;
    use crate::wallet::wallet::ActionWallet;
    use crate::utils::functions::*;
    use anyhow::Result;


    #[tokio::test]
    pub async fn create_asset() -> Result<()> {

        dotenvy::dotenv()?;

        let n = env::var("NETWORK")?;

        println!("Network {}", n);

        let mut wallet = ActionWallet::from_env();

        let res = wallet.execute(
            ContractCallInput::AssetFactory(
                AssetFactoryFunctionInput::CreateAsset(
                    CreateAssetArgs {
                        allow_list: 1,
                        acl_contract: "0x00000000000000000000000000000000006ca272".to_string(),
                        symbol: "CBR".to_string(),
                        name: "Cradle Base Reserve".to_string()
                    }
                )
            )
        ).await?;


        if let ContractCallOutput::AssetFactory(output) = res {
            if let AssetFactoryFunctionOutput::CreateAsset(o) = output {
                println!("Transaction ID:: {}", o.transaction_id);
            }
        }

        Ok(())
    }

}