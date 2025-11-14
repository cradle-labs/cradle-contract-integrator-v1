use crate::utils::functions::access_controller::{
    AccessControllerArgs, AccessControllerFunctionsInput, AccessControllerFunctionsOutput,
};
use crate::utils::functions::{ContractCallInput, ContractCallOutput};
use crate::utils::script_utils::BaseAssetConstructor;
use crate::utils::script_utils::DeployOrderBookSettler;
use crate::utils::script_utils::{
    AssetIssuerConstructor, AssetLendingPoolConstructor, CradleAccountFactoryConstructor,
    DeployLendingPoolFactory, GetClientArgs, NativeAssetIssuerConstructor,
};
use crate::wallet::wallet::ActionWallet;
use anyhow::{Result, format_err};
use chrono::Utc;
use clap::Parser;
use hedera::{
    Client, ContractCreateFlow, ContractCreateTransaction, ContractFunctionParameters, ContractId,
    FileAppendTransaction, FileCreateTransaction, FileId, Hbar,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};
use time::{Duration, OffsetDateTime};
use tokio::time::{Duration as TokioDuration, sleep};

#[derive(Deserialize, Serialize, Clone)]
pub struct Contract {
    pub name: String,
    pub bytecode: String,
    pub abi: Value,
    pub operator_account_id: String,
    pub operator_key: String,
    pub network: String,
    pub access_level: Option<u64>,
}

impl Contract {
    pub fn new(name: String, bytecode: String, abi: Value) -> Self {
        let args = GetClientArgs::parse();

        println!("Creating contract {}", name);

        let access_level: Option<u64> = {
            match name.as_str() {
                "AssetFactory" => Some(0),
                "CradleAccountFactory" => Some(0),
                "BaseAsset" => {
                    // might require its own factory
                    Some(0)
                }
                "BridgedAssetIssuer" => Some(1),
                "NativeAssetIssuer" => Some(1),
                "NativeAsset" => {
                    // might require its own factory
                    Some(1)
                }
                "AssetLendingPool" => Some(2),
                "CradleListingFactory" => Some(1),
                _ => None,
            }
        };

        Self {
            name,
            bytecode,
            abi,
            network: args.network,
            operator_account_id: args.operator_account_id.to_string(),
            operator_key: args.operator_key.to_string(),
            access_level,
        }
    }

    pub fn load_contract_from_file(name: String) -> Result<Self> {
        let file_name = format!("./contracts/out/{}.sol/{}.json", name.clone(), name.clone());
        let path = Path::new(&file_name);

        let (byte_code, abi) = match fs::read_to_string(path) {
            Ok(data) => {
                let map: HashMap<String, Value> = serde_json::from_str(&data)?;
                let abi = map.get("abi").unwrap();
                let bytecode = map.get("bytecode").unwrap();

                let bytecode_object = bytecode.as_object().unwrap();

                let compiled_code = bytecode_object.get("object").unwrap();
                let compiled_str = compiled_code.as_str().unwrap();

                (compiled_str.to_string(), abi.clone())
            }
            Err(_e) => panic!("Failed to load {}", name.clone()),
        };

        Ok(Self::new(name.clone(), byte_code, abi))
    }

    pub async fn get_client(&mut self, args: &GetClientArgs) -> Result<Client> {
        let client = Client::for_name(&args.network)?;
        let duration = Duration::minutes(10);
        client.set_request_timeout(Some(duration.try_into()?));
        client.set_max_backoff(TokioDuration::from_secs(30));
        client.set_max_attempts(5);

        client.set_operator(args.operator_account_id, args.operator_key.clone());

        Ok(client)
    }

    pub fn get_constructor_parameters(&mut self) -> Result<ContractFunctionParameters> {
        let mut params = ContractFunctionParameters::new();

        let values = match self.name.as_str() {
            "AccessController" => params,
            "AssetFactory" => {
                let acl_contract = env::var("ACL_CONTRACT")?;

                params.add_address(acl_contract.as_str());
                params
            }
            "CradleAccountFactory" => {
                let args = CradleAccountFactoryConstructor::try_parse()?;
                params.add_address(&args.acl_contract);
                params.add_uint64(args.allow_list);

                params
            }
            "BaseAsset" => {
                let args = BaseAssetConstructor::try_parse()?;
                params.add_string(&args.base_asset_name);
                params.add_string(&args.base_asset_symbol);
                params.add_address(&args.acl_contract);
                params.add_uint64(args.allow_list);

                params
            }
            "BridgedAssetIssuer" => {
                let args = AssetIssuerConstructor::try_parse()?;
                params.add_address(&args.treasury_address);
                params.add_address(&args.acl_contract);
                params.add_uint64(args.allow_list);
                params.add_address(&args.base_asset);

                params
            }
            "NativeAssetIssuer" => {
                let args = AssetIssuerConstructor::try_parse()?;
                params.add_address(&args.treasury_address);
                params.add_address(&args.acl_contract);
                params.add_uint64(args.allow_list);
                params.add_address(&args.base_asset);

                params
            }
            "NativeAsset" => {
                let args = NativeAssetIssuerConstructor::try_parse()?;

                println!("Native asset deployment args: {:?}", args);

                params.add_string("Cradle Native Reserve");
                params.add_string("CNR");
                params.add_address(&args.acl_contract);
                params.add_uint64(args.allow_list);

                params
            }
            "AssetLendingPool" => {
                let args = AssetLendingPoolConstructor::try_parse()?;
                params.add_uint64(args.ltv); // ltv
                params.add_uint64(args.optimal_utilization); // optimal utilization
                params.add_uint64(args.base_rate); // base rate
                params.add_uint64(args.slope1); // slope1
                params.add_uint64(args.slope2); // slope2
                params.add_uint64(args.liquidation_threshold); // liquidation threshold
                params.add_uint64(args.liquidation_discount); // liquidation discount
                params.add_uint64(args.reserve_factor); // reserve factor
                params.add_address(&args.lending.to_solidity_address()?); // lending
                params.add_string(&args.yield_asset); // yield asset
                params.add_string(&args.yield_asset_symbol); // yield asset symbol
                params.add_string(&args.lending_pool); // lending pool
                params.add_address(&args.acl_contract); // aclContract
                params.add_uint64(args.allow_list); // allow list

                params
            }
            "LendingPoolFactory" => {
                let args = DeployLendingPoolFactory::try_parse()?;

                params.add_address(args.acl_contract.as_str());

                params
            }
            "CradleOrderBookSettler" => {
                let args = DeployOrderBookSettler::try_parse()?;

                params.add_address(args.acl_contract.as_str());
                params.add_address(&args.order_book_treasury);

                params
            }
            "CradleListingFactory" => {
                let args = DeployLendingPoolFactory::try_parse()?;

                params.add_address(&args.acl_contract);

                params
            }
            _ => {
                return Err(format_err!(
                    "Contract deployment not supported yet, this is a local error :) "
                ));
            }
        };

        Ok(values)
    }

    pub async fn create_file(&mut self, args: &GetClientArgs, client: &Client) -> Result<FileId> {
        let expire_in_an_hour = OffsetDateTime::now_utc() + Duration::hours(1);
        let contents_full = self.bytecode.as_bytes();
        println!("File size :: {:?}", contents_full.len());
        const CHUNK_SIZE: usize = 1024;
        if contents_full.len() <= CHUNK_SIZE {
            let file_transaction_response = FileCreateTransaction::new()
                .keys([args.operator_key.public_key()])
                .contents(contents_full.to_vec())
                .max_transaction_fee(Hbar::new(2))
                .expiration_time(expire_in_an_hour.clone())
                .execute_with_timeout(&client, TokioDuration::from_secs(600))
                .await?;

            println!("transaction submitted...");

            let file_receipt = file_transaction_response.get_receipt(&client).await?;
            let new_file_id = file_receipt.file_id.unwrap();

            println!("Contract bytecode file {new_file_id}");
            Ok(new_file_id)
        } else {
            let mut offset = 0;
            let mut file_id: Option<FileId> = None;

            while offset < contents_full.len() {
                let end = std::cmp::min(offset + CHUNK_SIZE, contents_full.len());
                let chunk = &contents_full[offset..end];

                match file_id {
                    Some(fid) => {
                        let append_tx = FileAppendTransaction::new()
                            .file_id(fid)
                            .contents(chunk.to_vec())
                            .max_transaction_fee(Hbar::new(2))
                            .execute_with_timeout(&client, TokioDuration::from_secs(600))
                            .await?;

                        let receipt = append_tx.get_receipt(&client).await?;
                        // println!("Appended to file ID: {}", fid);
                        // println!("receipt: {:?}", receipt.transaction_id.unwrap());
                    }
                    None => {
                        let create_tx = FileCreateTransaction::new()
                            .keys([args.operator_key.public_key()])
                            .contents(chunk.to_vec())
                            .max_transaction_fee(Hbar::new(2))
                            .execute_with_timeout(&client, TokioDuration::from_secs(180))
                            .await?;

                        let receipt = create_tx.get_receipt(&client).await?;
                        let new_file_id = receipt.file_id.unwrap();
                        file_id = Some(new_file_id);
                        println!("Created file with ID: {}", new_file_id);
                    }
                }

                offset += chunk.len();
            }

            file_id.ok_or(format_err!("File id not found in file"))
        }
    }

    pub async fn deploy_contract(&mut self) -> Result<ContractId> {
        let args = GetClientArgs::try_parse()?;

        let client = self.get_client(&args).await?;

        println!("Client");

        let new_file_id = self.create_file(&args, &client).await?;
        //
        println!("Contract bytecode file {new_file_id}");

        let constructor_parameters = self.get_constructor_parameters()?;

        sleep(TokioDuration::from_secs(30)).await;

        let contract_id = ContractCreateTransaction::new()
            .admin_key(args.operator_key.public_key())
            .bytecode_file_id(new_file_id)
            .max_transaction_fee(Hbar::new(400))
            .constructor_parameters(constructor_parameters.to_bytes(None))
            .gas(15_000_000)
            .execute_with_timeout(&client, TokioDuration::from_secs(180))
            .await?
            .get_receipt(&client)
            .await?
            .contract_id
            .unwrap();

        println!("Contract id {contract_id}");

        if let Some(access_level) = self.access_level {
            println!("Needs to be added to access controller");

            let mut wallet = ActionWallet::from_env();

            let evm_address = contract_id.to_solidity_address()?;

            let res = wallet
                .execute(ContractCallInput::AccessController(
                    AccessControllerFunctionsInput::GrantAccess(AccessControllerArgs {
                        account: evm_address,
                        level: access_level,
                    }),
                ))
                .await?;

            if let ContractCallOutput::AccessController(
                AccessControllerFunctionsOutput::GrantAccess(output),
            ) = res
            {
                println!("Grant access transaction id {}", output.transaction_id);
            }
        }
        Ok(contract_id)
    }
}
