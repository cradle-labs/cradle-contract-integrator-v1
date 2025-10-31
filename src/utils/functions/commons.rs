use crate::wallet::wallet::ActionWallet;
use anyhow::Result;
use hedera::{AccountBalance, AccountBalanceQuery, AccountId, Client, ContractId, ContractInfoQuery};
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

    const MAX_RETRIES: u32 = 10;
    let mut attempt = 0;

    loop {
        attempt += 1;

        match client.get(&url).send().await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(body) => {
                        println!("Body {:?}", body.clone());

                        if let Some(contract_id_str) = body.get("contract_id").and_then(|v| v.as_str()) {
                            let contract_id = ContractId::from_str(contract_id_str)?;
                            return Ok(contract_id);
                        } else {
                            // Response was valid but missing contract_id - don't retry
                            anyhow::bail!(
                                "Failed to find contract_id in Mirror Node response for EVM address: {}",
                                evm_address
                            )
                        }
                    }
                    Err(e) if attempt < MAX_RETRIES => {
                        // Failed to parse JSON - retry with exponential backoff
                        let backoff_secs = 2u64.pow(attempt - 1);
                        println!("Attempt {} failed to parse response: {}. Retrying in {} seconds...", attempt, e, backoff_secs);
                        sleep(Duration::from_secs(backoff_secs)).await;
                        continue;
                    }
                    Err(e) => {
                        // Max retries exceeded
                        anyhow::bail!("Failed to get contract_id after {} attempts: {}", MAX_RETRIES, e)
                    }
                }
            }
            Err(e) if attempt < MAX_RETRIES => {
                // Request failed - retry with exponential backoff
                let backoff_secs = 2u64.pow(attempt - 1);
                println!("Attempt {} failed to fetch: {}. Retrying in {} seconds...", attempt, e, backoff_secs);
                sleep(Duration::from_secs(backoff_secs)).await;
                continue;
            }
            Err(e) => {
                // Max retries exceeded
                anyhow::bail!("Failed to fetch contract after {} attempts: {}", MAX_RETRIES, e)
            }
        }
    }
}

pub async fn get_account_balances(client: &Client,account_id: &str)->Result<AccountBalance> {
    let account_value = AccountId::from_str(account_id)?;
    let mut q = AccountBalanceQuery::new();
    q.account_id(account_value);

    let result = q.execute(client).await?;

    Ok(result)
}