use clap::Parser;
use hedera::ContractId;

#[derive(Parser, Debug)]
pub struct CradleContractIds {
    #[clap(long, env)]
    pub access_controller_contract_id: ContractId,
    #[clap(long, env)]
    pub bridged_asset_issuer_contract_id: ContractId,
    #[clap(long, env)]
    pub native_asset_issuer_contract_id: ContractId,
    #[clap(long, env)]
    pub cradle_account_factory_contract_id: ContractId,
    #[clap(long, env)]
    pub asset_lending_pool_contract_id: ContractId,
    #[clap(long, env)]
    pub asset_factory: ContractId,
    #[clap(long,env)]
    pub orderbook_settler: ContractId,
    #[clap(long, env)]
    pub asset_lending_pool_factory: ContractId
}