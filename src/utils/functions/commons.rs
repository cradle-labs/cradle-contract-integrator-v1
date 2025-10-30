use crate::wallet::wallet::ActionWallet;
use anyhow::Result;
use hedera::{ContractId, ContractInfoQuery};
use serde_json::json;
use std::str::FromStr;
use tokio::time::{Duration, sleep};

pub trait ContractFunctionProcessor<Output> {
    async fn process(&self, wallet: &mut ActionWallet)->Result<Output>;
}


pub async fn get_contract_id_from_evm_address(evm_address: &str) -> Result<ContractId> {
    sleep(Duration::from_secs(10)).await;
    // TODO: update to support mainnet and localhost
    let client = reqwest::Client::new();
    let url = format!(
        "https://testnet.mirrornode.hedera.com/api/v1/contracts/{}",
        evm_address
    );

    let response = client.get(&url).send().await?;
    let body = response.json::<serde_json::Value>().await?;

    println!("Body {:?}", body.clone());

    if let Some(contract_id_str) = body.get("contract_id").and_then(|v| v.as_str()) {
        let contract_id = ContractId::from_str(contract_id_str)?;
        Ok(contract_id)
    } else {
        anyhow::bail!(
            "Failed to find contract_id in Mirror Node response for EVM address: {}",
            evm_address
        )
    }
}