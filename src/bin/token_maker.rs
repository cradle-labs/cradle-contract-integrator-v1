use std::env;
use anyhow::Result;
use dialoguer::Input;
use hedera::ContractId;
use contract_integrator::utils::functions::asset_factory::{AssetFactoryFunctionInput, AssetFactoryFunctionOutput, CreateAssetArgs};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main()->Result<()>{

    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();
    let name: String = Input::new().with_prompt("Token Name").interact()?;
    let symbol: String = Input::new().with_prompt("Token Symbol").interact()?;

    let acl_contract:ContractId = env::var("ACCESS_CONTROLLER_CONTRACT_ID")?.parse()?;
    let allow_list: u64  = 1;


    let res = wallet.execute(
        ContractCallInput::AssetFactory(
            AssetFactoryFunctionInput::CreateAsset(
                CreateAssetArgs {
                    allow_list,
                    acl_contract: acl_contract.to_solidity_address().unwrap(),
                    name,
                    symbol
                }
            )
        )
    ).await?;

    if let ContractCallOutput::AssetFactory(AssetFactoryFunctionOutput::CreateAsset(output)) = res {

        println!("Transaction ID:: {:?}", output.transaction_id);
    }


    Ok(())
}