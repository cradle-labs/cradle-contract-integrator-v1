use crate::utils::functions::access_controller::{AccessControllerFunctionsInput, AccessControllerFunctionsOutput};
use crate::utils::functions::asset_factory::{AssetFactoryFunctionInput, AssetFactoryFunctionOutput};
use crate::utils::functions::asset_issuer::{AssetIssuerFunctionsInput, AssetIssuerFunctionsOutput};
use crate::utils::functions::asset_lending::{AssetLendingPoolFunctionsInput, AssetLendingPoolFunctionsOutput};
use crate::utils::functions::asset_manager::{AssetManagerFunctionInput, AssetManagerFunctionOutput};
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::utils::functions::cradle_account::{CradleAccountFunctionInput, CradleAccountFunctionOutput};
use crate::utils::functions::cradle_account_factory::{CradleAccountFactoryFunctionsInput, CradleAccountFactoryFunctionsOutput};
use crate::wallet::wallet::ActionWallet;

pub mod access_controller;
pub mod cradle_account_factory;
pub mod asset_issuer;
pub mod asset_lending;
pub mod cradle_account;
pub mod commons;
pub mod asset_manager;
pub mod asset_factory;

pub struct FunctionCallOutput<T> {
    pub transaction_id: String,
    pub output: Option<T>,
    // TODO: add additional fields that may be useful
}

pub enum ContractCallInput {
    AccessController(AccessControllerFunctionsInput),
    CradleAccountFactory(CradleAccountFactoryFunctionsInput),
    BridgedAssetIssuer(AssetIssuerFunctionsInput),
    NativeAssetIssuer(AssetIssuerFunctionsInput),
    AssetLendingPool(AssetLendingPoolFunctionsInput),
    CradleAccount(CradleAccountFunctionInput),
    AssetManager(AssetManagerFunctionInput),
    AssetFactory(AssetFactoryFunctionInput)
    // TODO: add cradle accounts
}



pub enum ContractCallOutput {
    AccessController(AccessControllerFunctionsOutput),
    CradleAccountFactory(CradleAccountFactoryFunctionsOutput),
    BridgedAssetIssuer(AssetIssuerFunctionsOutput),
    NativeAssetIssuer(AssetIssuerFunctionsOutput),
    AssetLendingPool(AssetLendingPoolFunctionsOutput),
    CradleAccount(CradleAccountFunctionOutput),
    AssetManager(AssetManagerFunctionOutput),
    AssetFactory(AssetFactoryFunctionOutput)
}


impl ContractFunctionProcessor<ContractCallOutput> for ContractCallInput {
    async fn process(&self, wallet: &mut ActionWallet) -> anyhow::Result<ContractCallOutput> {
        match self {
            ContractCallInput::AccessController(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AccessController(output))
            },
            ContractCallInput::CradleAccountFactory(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::CradleAccountFactory(output))
            },
            ContractCallInput::BridgedAssetIssuer(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::BridgedAssetIssuer(output))
            },
            ContractCallInput::NativeAssetIssuer(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::NativeAssetIssuer(output))
            },
            ContractCallInput::AssetLendingPool(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AssetLendingPool(output))
            },
            ContractCallInput::CradleAccount(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::CradleAccount(output))
            },
            ContractCallInput::AssetManager(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AssetManager(output))
            },
            ContractCallInput::AssetFactory(args)=>{
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AssetFactory(output))
            }
        }
    }
}