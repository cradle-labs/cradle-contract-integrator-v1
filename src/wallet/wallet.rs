use std::str::FromStr;
use hedera::{AccountId, Client, PrivateKey};
use anyhow::{anyhow, Result};
use crate::utils::functions::{ContractCallInput, ContractCallOutput};
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::utils::functions::cradle_account::CradleAccountFunctionInput;
use crate::wallet::contracts::CradleContractIds;
use clap::Parser;

#[derive(Parser)]
pub struct ActionWalletArgs {
    #[clap(long, env)]
    operator_account_id: String,
    #[clap(long, env)]
    operator_key: String,
    #[clap(long, env)]
    network: String
}

#[derive(Clone)]
pub struct ActionWallet {
    pub account_id: String,
    private_key: String,
    pub network: String,
    pub client: Client
}

impl ActionWallet {

    pub fn new(account_id: String, key: String, network: String) -> Self {
        let operator_account_id = AccountId::from_str(&account_id).unwrap();
        let operator_key = PrivateKey::from_str(&key).unwrap();

        let client = Client::for_name(&network).unwrap();
        client.set_operator(operator_account_id.clone(), operator_key.clone());

        Self {
            account_id,
            private_key: key,
            network,
            client
        }
    }
    
    
    pub fn from_env()->Self {
        let args = ActionWalletArgs::parse();
        
        Self::new(args.operator_account_id, args.operator_key, args.network)
    }

    pub fn get_contract_ids(&self) -> Result<CradleContractIds> {
        let ids = CradleContractIds::parse();
        Ok(ids)
    }


    pub async fn execute(&mut self, args: ContractCallInput)->Result<ContractCallOutput> {
        let mut wallet_clone = self.clone();
        let output = args.process(&mut wallet_clone).await?;
        Ok(output)
    }
}