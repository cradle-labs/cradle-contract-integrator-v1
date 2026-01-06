use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::utils::functions::cradle_account::CradleAccountFunctionInput;
use crate::utils::functions::{ContractCallInput, ContractCallOutput};
use crate::wallet::contracts::CradleContractIds;
use anyhow::{Result, anyhow};
use clap::Parser;
use hedera::{AccountId, Client, PrivateKey};
use std::iter;
use std::str::FromStr;

#[derive(Parser)]
pub struct ActionWalletArgs {
    #[clap(long, env)]
    operator_account_id: String,
    #[clap(long, env)]
    operator_key: String,
    #[clap(long, env)]
    network: String,
}

#[derive(Clone, Debug)]
pub struct ActionWallet {
    pub account_id: String,
    private_key: String,
    pub network: String,
    pub client: Client,
}

impl ActionWallet {
    pub fn new(account_id: String, key: String, network: String) -> Self {
        let operator_account_id = AccountId::from_str(&account_id).unwrap();
        let use_ed25519 =
            std::env::var("USE_ED25519").unwrap_or(String::from("false")) == String::from("true");
        let operator_key = if use_ed25519 {
            PrivateKey::from_str(&key).unwrap()
        } else {
            PrivateKey::from_str_ecdsa(&key).unwrap()
        };

        let client = Client::for_name(&network).unwrap();
        client.set_operator(operator_account_id.clone(), operator_key.clone());

        Self {
            account_id,
            private_key: key,
            network,
            client,
        }
    }

    pub fn from_env() -> Self {
        #[cfg(test)]
        let args = ActionWalletArgs::parse_from(iter::empty::<String>());

        #[cfg(not(test))]
        let args = ActionWalletArgs::parse();

        Self::new(args.operator_account_id, args.operator_key, args.network)
    }

    pub fn get_contract_ids(&self) -> Result<CradleContractIds> {
        #[cfg(test)]
        let ids = CradleContractIds::parse_from(iter::empty::<String>());

        #[cfg(not(test))]
        let ids = CradleContractIds::parse();

        Ok(ids)
    }

    pub async fn execute(&mut self, args: ContractCallInput) -> Result<ContractCallOutput> {
        let mut wallet_clone = self.clone();
        let output = args.process(&mut wallet_clone).await?;
        Ok(output)
    }
}
