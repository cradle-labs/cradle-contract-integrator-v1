use hedera::{AccountId, PrivateKey};
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct GetClientArgs {
    #[clap(long,env)]
    pub operator_account_id: AccountId,
    #[clap(long,env)]
    pub operator_key: PrivateKey,
    #[clap(long,env, default_value = "testnet")]
    pub network: String,
}


#[derive(Parser, Debug)]
pub struct CradleAccountFactoryConstructor {
    #[clap(long,env)]
    pub acl_contract: String,
    #[clap(long,env)]
    pub allow_list: u64
}

#[derive(Parser, Debug)]
pub struct AssetIssuerConstructor {
    #[clap(long,env)]
    pub acl_contract: String,
    #[clap(long,env)]
    pub allow_list: u64,
    #[clap(long,env)]
    pub reserve_token: String
}

#[derive(Parser, Debug)]
pub struct NativeAssetIssuerConstructor {
    #[clap(long,env)]
    pub acl_contract: String,
    #[clap(long,env)]
    pub allow_list: u64,
}

#[derive(Parser, Debug)]
pub struct AssetLendingPoolConstructor {
    #[clap(long,env)]
    pub ltv: u64,
    #[clap(long,env)]
    pub optimal_utilization: u64,
    #[clap(long,env)]
    pub base_rate: u64,
    #[clap(long,env)]
    pub slope1: u64,
    #[clap(long,env)]
    pub slope2: u64,
    #[clap(long,env)]
    pub liquidation_threshold: u64,
    #[clap(long,env)]
    pub liquidation_discount: u64,
    #[clap(long,env)]
    pub reserve_factor: u64,
    #[clap(long,env)]
    pub lending: String,
    #[clap(long,env)]
    pub yield_asset: String,
    #[clap(long,env)]
    pub yield_asset_symbol: String,
    #[clap(long,env)]
    pub lending_pool: String,
    #[clap(long,env)]
    pub acl_contract: String,
    #[clap(long,env)]
    pub allow_list: u64
}


#[derive(Parser, Debug)]
pub struct BaseAssetConstructor {
    #[clap(long,env)]
    pub base_asset_name: String,
    #[clap(long,env)]
    pub base_asset_symbol: String,
    #[clap(long,env)]
    pub acl_contract: String,
    #[clap(long,env)]
    pub allow_list: u64
}