use serde::{Deserialize, Serialize};

use crate::utils::functions::access_controller::{
    AccessControllerFunctionsInput, AccessControllerFunctionsOutput,
};
use crate::utils::functions::asset_factory::{
    AssetFactoryFunctionInput, AssetFactoryFunctionOutput,
};
use crate::utils::functions::asset_issuer::{
    AssetIssuerFunctionsInput, AssetIssuerFunctionsOutput,
};
use crate::utils::functions::asset_lending::{
    AssetLendingPoolFunctionsInput, AssetLendingPoolFunctionsOutput,
};
use crate::utils::functions::asset_lending_pool_factory::{
    AssetLendingPoolFactoryFunctionInput, AssetLendingPoolFactoryFunctionOutput,
};
use crate::utils::functions::asset_manager::{
    AssetManagerFunctionInput, AssetManagerFunctionOutput,
};
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::utils::functions::cradle_account::{
    CradleAccountFunctionInput, CradleAccountFunctionOutput,
};
use crate::utils::functions::cradle_account_factory::{
    CradleAccountFactoryFunctionsInput, CradleAccountFactoryFunctionsOutput,
};
use crate::utils::functions::cradle_native_listing::{
    CradleNativeListingFunctionsInput, CradleNativeListingFunctionsOutput,
};
use crate::utils::functions::listing_factory::{
    CradleListingFactoryFunctionsInput, CradleListingFactoryFunctionsOutput,
};
use crate::utils::functions::orderbook_settler::{
    OrderBookSettlerFunctionInput, OrderBookSettlerFunctionOutput,
};
use crate::wallet::wallet::ActionWallet;

pub mod access_controller;
pub mod asset_factory;
pub mod asset_issuer;
pub mod asset_lending;
pub mod asset_lending_pool_factory;
pub mod asset_manager;
pub mod commons;
pub mod cradle_account;
pub mod cradle_account_factory;
pub mod cradle_native_listing;
pub mod listing_factory;
pub mod orderbook_settler;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionCallOutput<T> {
    pub transaction_id: String,
    pub output: Option<T>,
    // TODO: add additional fields that may be useful
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WithContractId<Rest> {
    pub contract_id: String,
    pub rest: Option<Rest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ContractCallInput {
    AccessController(AccessControllerFunctionsInput),
    CradleAccountFactory(CradleAccountFactoryFunctionsInput),
    BridgedAssetIssuer(AssetIssuerFunctionsInput),
    NativeAssetIssuer(AssetIssuerFunctionsInput),
    AssetLendingPool(AssetLendingPoolFunctionsInput),
    CradleAccount(CradleAccountFunctionInput),
    AssetManager(AssetManagerFunctionInput),
    AssetFactory(AssetFactoryFunctionInput),
    OrderBookSettler(OrderBookSettlerFunctionInput),
    AssetLendingPoolFactory(AssetLendingPoolFactoryFunctionInput),
    CradleListingFactory(CradleListingFactoryFunctionsInput),
    CradleNativeListing(CradleNativeListingFunctionsInput),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ContractCallOutput {
    AccessController(AccessControllerFunctionsOutput),
    CradleAccountFactory(CradleAccountFactoryFunctionsOutput),
    BridgedAssetIssuer(AssetIssuerFunctionsOutput),
    NativeAssetIssuer(AssetIssuerFunctionsOutput),
    AssetLendingPool(AssetLendingPoolFunctionsOutput),
    CradleAccount(CradleAccountFunctionOutput),
    AssetManager(AssetManagerFunctionOutput),
    AssetFactory(AssetFactoryFunctionOutput),
    OrderBookSettler(OrderBookSettlerFunctionOutput),
    AssetLendingPoolFactory(AssetLendingPoolFactoryFunctionOutput),
    CradleListingFactory(CradleListingFactoryFunctionsOutput),
    CradleNativeListing(CradleNativeListingFunctionsOutput),
}

impl ContractFunctionProcessor<ContractCallOutput> for ContractCallInput {
    async fn process(&self, wallet: &mut ActionWallet) -> anyhow::Result<ContractCallOutput> {
        match self {
            ContractCallInput::AccessController(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AccessController(output))
            }
            ContractCallInput::CradleAccountFactory(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::CradleAccountFactory(output))
            }
            ContractCallInput::BridgedAssetIssuer(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::BridgedAssetIssuer(output))
            }
            ContractCallInput::NativeAssetIssuer(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::NativeAssetIssuer(output))
            }
            ContractCallInput::AssetLendingPool(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AssetLendingPool(output))
            }
            ContractCallInput::CradleAccount(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::CradleAccount(output))
            }
            ContractCallInput::AssetManager(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AssetManager(output))
            }
            ContractCallInput::AssetFactory(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AssetFactory(output))
            }
            ContractCallInput::OrderBookSettler(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::OrderBookSettler(output))
            }
            ContractCallInput::AssetLendingPoolFactory(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::AssetLendingPoolFactory(output))
            }
            ContractCallInput::CradleListingFactory(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::CradleListingFactory(output))
            }
            ContractCallInput::CradleNativeListing(args) => {
                let output = args.process(wallet).await?;
                Ok(ContractCallOutput::CradleNativeListing(output))
            }
        }
    }
}
