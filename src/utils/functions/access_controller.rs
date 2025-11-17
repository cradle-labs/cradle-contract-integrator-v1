use crate::utils::functions::FunctionCallOutput;
use crate::utils::functions::access_controller::AccessControllerFunctionsInput::HasAccess;
use crate::utils::functions::commons::ContractFunctionProcessor;
use crate::wallet::wallet::ActionWallet;
use hedera::{ContractCallQuery, ContractExecuteTransaction, ContractFunctionParameters};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessControllerArgs {
    pub level: u64,
    pub account: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrantAccessBatchArgs {
    pub level: u64,
    pub accounts: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClearLevelArgs {
    pub level: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetLevelArgs {
    pub level: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RotateAdminArgs {
    pub old_key: String,
    pub new_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AccessControllerFunctionsInput {
    HasAccess(AccessControllerArgs),
    GrantAccess(AccessControllerArgs),
    RevokeAccess(AccessControllerArgs),
    GrantAccessBatch(GrantAccessBatchArgs),
    ClearLevel(ClearLevelArgs),
    GetLevel(GetLevelArgs),
    RotateAdmin(RotateAdminArgs),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HasAccessOutput {
    pub has_access: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AccessControllerFunctionsOutput {
    HasAccess(FunctionCallOutput<HasAccessOutput>),
    GrantAccess(FunctionCallOutput<()>),
    RevokeAccess(FunctionCallOutput<()>),
    GrantAccessBatch(FunctionCallOutput<()>),
    ClearLevel(FunctionCallOutput<()>),
    GetLevel(FunctionCallOutput<Vec<String>>),
    RotateAdmin(FunctionCallOutput<()>),
}

impl ContractFunctionProcessor<AccessControllerFunctionsOutput> for AccessControllerFunctionsInput {
    async fn process(
        &self,
        wallet: &mut ActionWallet,
    ) -> anyhow::Result<AccessControllerFunctionsOutput> {
        let contract_ids = wallet.get_contract_ids()?;
        let mut transaction = ContractExecuteTransaction::new();
        transaction.contract_id(contract_ids.access_controller_contract_id);
        transaction.gas(10_000_000);

        let mut query_transaction = ContractCallQuery::new();
        query_transaction.contract_id(contract_ids.access_controller_contract_id);
        query_transaction.gas(1_000_000);

        match &self {
            HasAccess(args) => {
                query_transaction.function("hasAccess");
                let mut params = ContractFunctionParameters::new();

                params.add_uint64(args.level);
                params.add_address(args.account.as_str());

                query_transaction.function_parameters(params.to_bytes(Some("hasAccess")));

                let response = query_transaction
                    .execute_with_timeout(&mut wallet.client, Duration::from_secs(180))
                    .await?;

                let has_access = response.get_bool(0).unwrap();

                let output = FunctionCallOutput {
                    transaction_id: "".to_string(),
                    output: Some(HasAccessOutput { has_access }),
                };
                Ok(AccessControllerFunctionsOutput::HasAccess(output))
            }
            AccessControllerFunctionsInput::GrantAccess(args) => {
                transaction.function("grantAccess");
                let mut params = ContractFunctionParameters::new();

                params.add_uint64(args.level);
                params.add_address(args.account.as_str());

                transaction.function_parameters(params.to_bytes(Some("grantAccess")));

                let response = transaction.execute(&mut wallet.client).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AccessControllerFunctionsOutput::GrantAccess(output))
            }
            AccessControllerFunctionsInput::RevokeAccess(args) => {
                transaction.function("revokeAccess");
                let mut params = ContractFunctionParameters::new();

                params.add_uint64(args.level);
                params.add_address(args.account.as_str());

                transaction.function_parameters(params.to_bytes(Some("revokeAccess")));

                let response = transaction.execute(&mut wallet.client).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AccessControllerFunctionsOutput::RevokeAccess(output))
            }
            AccessControllerFunctionsInput::GrantAccessBatch(args) => {
                transaction.function("grantAccessBatch");
                let mut params = ContractFunctionParameters::new();

                params.add_uint64(args.level);
                let addresses: Vec<&str> = args.accounts.iter().map(|s| s.as_str()).collect();
                params.add_address_array(&addresses);

                transaction.function_parameters(params.to_bytes(Some("grantAccessBatch")));

                let response = transaction.execute(&mut wallet.client).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AccessControllerFunctionsOutput::GrantAccessBatch(output))
            }
            AccessControllerFunctionsInput::ClearLevel(args) => {
                transaction.function("clearLevel");
                let mut params = ContractFunctionParameters::new();

                params.add_uint64(args.level);
                transaction.function_parameters(params.to_bytes(Some("clearLevel")));

                let response = transaction.execute(&mut wallet.client).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AccessControllerFunctionsOutput::ClearLevel(output))
            }
            AccessControllerFunctionsInput::GetLevel(args) => {
                query_transaction.function("getLevel");
                let mut params = ContractFunctionParameters::new();

                params.add_uint64(args.level);

                query_transaction.function_parameters(params.to_bytes(Some("getLevel")));

                let response = query_transaction.execute(&mut wallet.client).await?;

                // let receipt = response.get;
                todo!("Seems address array is not supported yet in hedera sdk");
                // let output = FunctionCallOutput {
                //     transaction_id: receipt.transaction_id.unwrap().to_string(),
                //     output: None
                // };
                //
                // Ok(AccessControllerFunctionsOutput::GetLevel( output))
            }
            AccessControllerFunctionsInput::RotateAdmin(args) => {
                transaction.function("rotateLevel0Key");

                let mut params = ContractFunctionParameters::new();
                params.add_address(args.old_key.as_str());
                params.add_address(args.new_key.as_str());

                transaction.function_parameters(params.to_bytes(Some("rotateLevel0Key")));

                let response = transaction.execute(&mut wallet.client).await?;

                let receipt = response.get_receipt(&mut wallet.client).await?;

                let output = FunctionCallOutput {
                    transaction_id: receipt.transaction_id.unwrap().to_string(),
                    output: None,
                };

                Ok(AccessControllerFunctionsOutput::RotateAdmin(output))
            }
        }
    }
}
