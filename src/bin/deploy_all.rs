use anyhow::Result;
use chrono::Utc;
use contract_integrator::utils::contract::Contract;
use contract_integrator::utils::functions::asset_factory::{
    AssetFactoryFunctionInput, AssetFactoryFunctionOutput, CreateAssetArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;
use dialoguer::{Confirm, Input};
use hedera::ContractId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct DeploymentStep {
    name: String,
    contract_name: String,
    env_var: String,
    order: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DeploymentStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy)]
enum DeploymentMode {
    Full,
    ContractsOnly,
    TokensOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContractDeploymentState {
    order: usize,
    name: String,
    contract_name: String,
    env_var: String,
    status: DeploymentStatus,
    contract_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenState {
    name: String,
    status: DeploymentStatus,
    id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeploymentState {
    started_at: String,
    last_updated: String,
    deployments: Vec<ContractDeploymentState>,
    tokens: Vec<TokenState>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Load or create deployment state
    let mut state = load_or_create_state()?;
    let state_path = get_state_file_path();
    let has_existing_state = state_path.exists()
        && state
            .deployments
            .iter()
            .any(|d| !matches!(d.status, DeploymentStatus::Pending))
        || state
            .tokens
            .iter()
            .any(|t| !matches!(t.status, DeploymentStatus::Pending));

    if has_existing_state {
        println!("\n⚠️  Found existing deployment state");
        let resume_option = {
            let options = vec![
                "Resume from failed deployments",
                "Redeploy specific contracts",
                "Start fresh deployment",
            ];
            dialoguer::Select::new()
                .items(&options)
                .default(0)
                .interact()?
        };

        match resume_option {
            0 => {
                println!("Resuming from previous deployment...");
                apply_state_to_env(&state);
            }
            1 => {
                println!("\nSelect contracts to redeploy:");
                let contract_names: Vec<String> = state
                    .deployments
                    .iter()
                    .map(|d| format!("{} ({})", d.name, d.contract_name))
                    .collect();

                let selected = dialoguer::MultiSelect::new()
                    .items(&contract_names)
                    .interact()?;

                if !selected.is_empty() {
                    println!("\nResetting selected contracts to pending...");
                    for idx in selected {
                        if idx < state.deployments.len() {
                            state.deployments[idx].status = DeploymentStatus::Pending;
                            state.deployments[idx].contract_id = None;
                        }
                    }
                    save_state(&state)?;
                    apply_state_to_env(&state);
                } else {
                    println!("No contracts selected, resuming normally...");
                    apply_state_to_env(&state);
                }
            }
            _ => {
                println!("Starting fresh deployment...");
                state = create_initial_state();
            }
        }
    }

    // Ask user what they want to deploy
    let deployment_mode = {
        println!("\nWhat would you like to deploy?");
        let selection = dialoguer::Select::new()
            .items(&[
                "Full deployment (contracts + tokens)",
                "Contracts only",
                "Tokens only",
            ])
            .default(0)
            .interact()?;

        match selection {
            0 => DeploymentMode::Full,
            1 => DeploymentMode::ContractsOnly,
            2 => DeploymentMode::TokensOnly,
            _ => DeploymentMode::Full,
        }
    };

    // Define deployment steps
    let steps = vec![
        DeploymentStep {
            name: "Access Controller".to_string(),
            contract_name: "AccessController".to_string(),
            env_var: "ACCESS_CONTROLLER_CONTRACT_ID".to_string(),
            order: 1,
        },
        DeploymentStep {
            name: "Asset Factory".to_string(),
            contract_name: "AssetFactory".to_string(),
            env_var: "ASSET_FACTORY".to_string(),
            order: 2,
        },
        DeploymentStep {
            name: "Cradle Account Factory".to_string(),
            contract_name: "CradleAccountFactory".to_string(),
            env_var: "CRADLE_ACCOUNT_FACTORY_CONTRACT_ID".to_string(),
            order: 3,
        },
        DeploymentStep {
            name: "Bridged Asset Issuer".to_string(),
            contract_name: "BridgedAssetIssuer".to_string(),
            env_var: "BRIDGED_ASSET_ISSUER_CONTRACT_ID".to_string(),
            order: 4,
        },
        DeploymentStep {
            name: "Native Asset Issuer".to_string(),
            contract_name: "NativeAssetIssuer".to_string(),
            env_var: "NATIVE_ASSET_ISSUER_CONTRACT_ID".to_string(),
            order: 5,
        },
        DeploymentStep {
            name: "Cradle Order Book Settler".to_string(),
            contract_name: "CradleOrderBookSettler".to_string(),
            env_var: "CRADLE_ORDER_BOOK_SETTLER_CONTRACT_ID".to_string(),
            order: 6,
        },
        DeploymentStep {
            name: "Lending Pool Factory".to_string(),
            contract_name: "LendingPoolFactory".to_string(),
            env_var: "ASSET_LENDING_POOL_FACTORY".to_string(),
            order: 7,
        },
        DeploymentStep {
            name: "Listings Factory".to_string(),
            contract_name: "CradleListingFactory".to_string(),
            env_var: "CRADLE_LISTING_FACTORY_CONTRACT_ID".to_string(),
            order: 8,
        },
    ];

    println!("\n╔════════════════════════════════════════╗");
    println!("║   Contract Deployment Orchestrator    ║");
    println!("║         Interactive Deployer          ║");
    println!("╚════════════════════════════════════════╝\n");

    // Show deployment plan based on mode
    match deployment_mode {
        DeploymentMode::Full => {
            println!("Deployment Plan (8 steps):");
            println!("  1. Access Controller");
            println!("  2. Asset Factory");
            println!("  2.5. Create Base Asset Token");
            println!("  2.5. Create Yield Asset Token");
            println!("  3. Cradle Account Factory");
            println!("  4. Bridged Asset Issuer");
            println!("  5. Native Asset Issuer");
            println!("  6. Cradle Order Book Settler");
            println!("  7. Lending Pool Factory");
            println!("  8. Cradle Listing Factory");
        }
        DeploymentMode::ContractsOnly => {
            println!("Deployment Plan (7 contract steps):");
            println!("  1. Access Controller");
            println!("  2. Asset Factory");
            println!("  3. Cradle Account Factory");
            println!("  4. Bridged Asset Issuer");
            println!("  5. Native Asset Issuer");
            println!("  6. Cradle Order Book Settler");
            println!("  7. Lending Pool Factory");
            println!("  8. Cradle Listing Factory");
        }
        DeploymentMode::TokensOnly => {
            println!("Deployment Plan (token creation):");
            println!("  1. Create Base Asset Token");
            println!("  2. Create Yield Asset Token");
        }
    }
    println!();

    // Confirm start
    let prompt_text = match deployment_mode {
        DeploymentMode::Full => "Ready to begin deployment?",
        DeploymentMode::ContractsOnly => "Ready to deploy contracts?",
        DeploymentMode::TokensOnly => "Ready to create tokens?",
    };

    if !Confirm::new().with_prompt(prompt_text).interact()? {
        println!("Deployment cancelled.");
        return Ok(());
    }

    // Store deployed contract IDs
    let mut deployed_ids: HashMap<String, String> = HashMap::new();

    // Skip already completed deployments when resuming
    for deployment in &state.deployments {
        if let Some(contract_id) = &deployment.contract_id {
            deployed_ids.insert(deployment.env_var.clone(), contract_id.clone());
        }
    }
    for token in &state.tokens {
        if let Some(id) = &token.id {
            deployed_ids.insert(token.name.clone(), id.clone());
        }
    }

    // Deploy contracts 1-7 (including Lending Pool Factory) - skip if TokensOnly mode
    if !matches!(deployment_mode, DeploymentMode::TokensOnly) {
        for (_idx, step) in steps.iter().enumerate() {
            // Skip if already completed
            let deployment_state = state.deployments.iter().find(|d| d.env_var == step.env_var);
            if let Some(ds) = deployment_state {
                if matches!(ds.status, DeploymentStatus::Completed) {
                    println!("\n✓ {} already deployed (skipping)", step.name);
                    continue;
                }
            }
            // After Asset Factory (step 2), create tokens
            if step.order == 2 {
                println!(
                    "\n┌─ Step {} of 7 ─────────────────────────────────┐",
                    step.order
                );
                println!("│ Deploying: {:<35} │", step.name);
                println!("└───────────────────────────────────────────────┘");

                // Deploy Asset Factory with unlimited retries
                if deploy_contract_with_unlimited_retries(step, &mut deployed_ids, &mut state)
                    .await?
                {
                    // Now create tokens immediately after Asset Factory (only if not ContractsOnly mode)
                    if !matches!(deployment_mode, DeploymentMode::ContractsOnly) {
                        println!("\n┌─ Step 2.5 of 7 ───────────────────────────────┐");
                        println!("│ Creating Base Asset & Yield Asset Tokens      │");
                        println!("└───────────────────────────────────────────────┘");

                        loop {
                            match create_tokens(&mut deployed_ids).await {
                                Ok((base_id, yield_id)) => {
                                    // Store token IDs
                                    unsafe {
                                        env::set_var("BASE_ASSET", base_id.clone());
                                        env::set_var("YIELD_ASSET", yield_id.clone());
                                    }
                                    deployed_ids.insert("BASE_ASSET".to_string(), base_id.clone());
                                    deployed_ids
                                        .insert("YIELD_ASSET".to_string(), yield_id.clone());
                                    let reserve_id =
                                        ContractId::from_solidity_address(base_id.as_str())?
                                            .to_string();
                                    unsafe {
                                        env::set_var("RESERVE_ASSET_ID", reserve_id.clone());
                                    }
                                    deployed_ids.insert("RESERVE_ASSET_ID".to_string(), reserve_id);

                                    update_token_status(
                                        &mut state,
                                        "BASE_ASSET",
                                        DeploymentStatus::Completed,
                                        Some(base_id.clone()),
                                    );
                                    update_token_status(
                                        &mut state,
                                        "YIELD_ASSET",
                                        DeploymentStatus::Completed,
                                        Some(yield_id.clone()),
                                    );
                                    if let Some(reserve_id_val) =
                                        deployed_ids.get("RESERVE_ASSET_ID")
                                    {
                                        update_token_status(
                                            &mut state,
                                            "RESERVE_ASSET_ID",
                                            DeploymentStatus::Completed,
                                            Some(reserve_id_val.clone()),
                                        );
                                    }
                                    save_state(&state)?;
                                    break;
                                }
                                Err(e) => {
                                    println!("  ✗ Token creation failed: {}", e);
                                    if !Confirm::new()
                                        .with_prompt("Retry token creation?")
                                        .interact()?
                                    {
                                        println!("  Skipping token creation...");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Standard deployment for other steps with unlimited retries
                println!(
                    "\n┌─ Step {} of 7 ─────────────────────────────────┐",
                    step.order
                );
                println!("│ Deploying: {:<35} │", step.name);
                println!("└───────────────────────────────────────────────┘");

                let _ = deploy_contract_with_unlimited_retries(step, &mut deployed_ids, &mut state)
                    .await;
            }
        }
    }

    // For TokensOnly or Full mode with tokens, create tokens if Asset Factory was deployed or already exists
    if matches!(
        deployment_mode,
        DeploymentMode::Full | DeploymentMode::TokensOnly
    ) {
        if deployed_ids.contains_key("ASSET_FACTORY") || env::var("ASSET_FACTORY").is_ok() {
            println!("\n┌─ Token Creation ───────────────────────────────┐");
            println!("│ Creating Base Asset & Yield Asset Tokens      │");
            println!("└───────────────────────────────────────────────┘");

            loop {
                match create_tokens(&mut deployed_ids).await {
                    Ok((base_id, yield_id)) => {
                        // Store token IDs
                        unsafe {
                            env::set_var("BASE_ASSET", base_id.clone());
                            env::set_var("YIELD_ASSET", yield_id.clone());
                        }
                        deployed_ids.insert("BASE_ASSET".to_string(), base_id.clone());
                        deployed_ids.insert("YIELD_ASSET".to_string(), yield_id.clone());
                        let reserve_id =
                            ContractId::from_solidity_address(base_id.as_str())?.to_string();
                        unsafe {
                            env::set_var("RESERVE_ASSET_ID", reserve_id.clone());
                        }
                        deployed_ids.insert("RESERVE_ASSET_ID".to_string(), reserve_id);

                        update_token_status(
                            &mut state,
                            "BASE_ASSET",
                            DeploymentStatus::Completed,
                            Some(base_id.clone()),
                        );
                        update_token_status(
                            &mut state,
                            "YIELD_ASSET",
                            DeploymentStatus::Completed,
                            Some(yield_id.clone()),
                        );
                        if let Some(reserve_id_val) = deployed_ids.get("RESERVE_ASSET_ID") {
                            update_token_status(
                                &mut state,
                                "RESERVE_ASSET_ID",
                                DeploymentStatus::Completed,
                                Some(reserve_id_val.clone()),
                            );
                        }
                        save_state(&state)?;
                        break;
                    }
                    Err(e) => {
                        println!("  ✗ Token creation failed: {}", e);
                        if !Confirm::new()
                            .with_prompt("Retry token creation?")
                            .interact()?
                        {
                            println!("  Skipping token creation...");
                            break;
                        }
                    }
                }
            }
        } else if matches!(deployment_mode, DeploymentMode::TokensOnly) {
            println!("\n✗ Asset Factory contract not found!");
            println!(
                "  Deploy contracts first or ensure ASSET_FACTORY environment variable is set."
            );
            return Ok(());
        }
    }

    // Display summary
    println!("\n╔════════════════════════════════════════╗");
    println!("║        Deployment Summary              ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("Deployed Contracts & Tokens:");
    for (env_var, contract_id) in &deployed_ids {
        println!("  {} = {}", env_var, contract_id);
    }

    // Ask to update .env
    if !deployed_ids.is_empty() {
        println!();
        if Confirm::new()
            .with_prompt("Update .env file with deployed contract IDs?")
            .interact()?
        {
            match update_env_file(&deployed_ids) {
                Ok(_) => {
                    println!("\n✓ .env file updated successfully!");
                }
                Err(e) => {
                    println!("\n✗ Failed to update .env file: {}", e);
                    println!("\nManually add these to your .env file:");
                    for (env_var, contract_id) in &deployed_ids {
                        println!("{}={}", env_var, contract_id);
                    }
                }
            }
        }
    }

    println!("\n✓ Deployment orchestration complete!");
    Ok(())
}

async fn create_tokens(deployed_ids: &mut HashMap<String, String>) -> Result<(String, String)> {
    let mut wallet = ActionWallet::from_env();

    // Get Access Controller contract ID from deployed contracts or env
    let acl_contract_str: String = deployed_ids
        .get("ACCESS_CONTROLLER_CONTRACT_ID")
        .cloned()
        .unwrap_or_else(|| env::var("ACCESS_CONTROLLER_CONTRACT_ID").unwrap_or_default());

    // Parse string to ContractId and convert to Solidity address
    let acl_contract_id = ContractId::from_str(&acl_contract_str)?;
    let acl_contract_solidity = acl_contract_id.to_solidity_address()?;

    let allow_list: u64 = 1;

    // Create Base Asset
    println!("┌─ Creating Base Asset Token ────────────────┐");
    let base_name: String = Input::new().with_prompt("Base Asset Name").interact()?;
    let base_symbol: String = Input::new().with_prompt("Base Asset Symbol").interact()?;

    println!("  ⏳ Creating Base Asset...");
    let base_asset_result = wallet
        .execute(ContractCallInput::AssetFactory(
            AssetFactoryFunctionInput::CreateAsset(CreateAssetArgs {
                name: base_name,
                symbol: base_symbol,
                acl_contract: acl_contract_solidity.clone(),
                allow_list,
            }),
        ))
        .await?;

    let base_asset_address =
        if let ContractCallOutput::AssetFactory(AssetFactoryFunctionOutput::CreateAsset(output)) =
            base_asset_result
        {
            println!("  ✓ Base Asset created!");
            println!("  📋 Transaction ID: {}", output.transaction_id);

            let asset_manager_address = output.output.as_ref().unwrap().asset_manager.clone();
            println!("  🛠️  Asset Manager Address: {}", asset_manager_address);

            unsafe {
                env::set_var("BASE_ASSET_MANAGER", asset_manager_address.clone());
                deployed_ids.insert("BASE_ASSET_MANAGER".to_string(), asset_manager_address);
            }

            output.output.unwrap().token
        } else {
            anyhow::bail!("Unexpected response from Asset Factory")
        };

    println!("└────────────────────────────────────────────┘\n");

    // Create Yield Asset
    println!("┌─ Creating Yield Asset Token ───────────────┐");
    let yield_name: String = Input::new().with_prompt("Yield Asset Name").interact()?;
    let yield_symbol: String = Input::new().with_prompt("Yield Asset Symbol").interact()?;

    println!("  ⏳ Creating Yield Asset...");
    let yield_asset_result = wallet
        .execute(ContractCallInput::AssetFactory(
            AssetFactoryFunctionInput::CreateAsset(CreateAssetArgs {
                name: yield_name,
                symbol: yield_symbol,
                acl_contract: acl_contract_solidity.clone(),
                allow_list,
            }),
        ))
        .await?;

    let yield_asset_address =
        if let ContractCallOutput::AssetFactory(AssetFactoryFunctionOutput::CreateAsset(output)) =
            yield_asset_result
        {
            println!("  ✓ Yield Asset created!");
            println!("  📋 Transaction ID: {}", output.transaction_id);

            let asset_manager_address = output.output.as_ref().unwrap().asset_manager.clone();
            println!("  🛠️  Asset Manager Address: {}", asset_manager_address);

            unsafe {
                env::set_var("YIELD_ASSET_MANAGER", asset_manager_address.clone());
                deployed_ids.insert("YIELD_ASSET_MANAGER".to_string(), asset_manager_address);
            }

            output.output.unwrap().token
        } else {
            anyhow::bail!("Unexpected response from Asset Factory")
        };

    let reserve_asset_id =
        ContractId::from_solidity_address(base_asset_address.as_str())?.to_string();
    unsafe {
        env::set_var("RESERVE_ASSET_ID", reserve_asset_id.clone());
        println!("Reserve asset id set to {}", env::var("RESERVE_ASSET_ID")?);
    }
    deployed_ids.insert("RESERVE_ASSET_ID".to_string(), reserve_asset_id);

    println!("└────────────────────────────────────────────┘\n");

    println!("✓ Tokens created successfully:");
    println!("  Base Asset: {}", base_asset_address);
    println!("  Yield Asset: {}", yield_asset_address);

    Ok((base_asset_address, yield_asset_address))
}

fn update_env_file(deployed_ids: &HashMap<String, String>) -> Result<()> {
    let env_path = ".env";
    let backup_path = ".env.backup";

    // Create backup
    if Path::new(env_path).exists() {
        fs::copy(env_path, backup_path)?;
        println!("  📁 Backup created: {}", backup_path);
    }

    // Read current .env file
    let mut content = if Path::new(env_path).exists() {
        fs::read_to_string(env_path)?
    } else {
        String::new()
    };

    // Update or add env variables
    for (env_var, contract_id) in deployed_ids {
        let line_pattern = format!("{}=", env_var);

        if content.contains(&line_pattern) {
            // Replace existing line
            let lines: Vec<&str> = content.lines().collect();
            let updated_lines: Vec<String> = lines
                .iter()
                .map(|line| {
                    if line.starts_with(&line_pattern) {
                        format!("{}={}", env_var, contract_id)
                    } else {
                        line.to_string()
                    }
                })
                .collect();
            content = updated_lines.join("\n");
            if !content.ends_with('\n') {
                content.push('\n');
            }
        } else {
            // Add new line
            if !content.ends_with('\n') && !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&format!("{}={}\n", env_var, contract_id));
        }
    }

    // Write updated content
    fs::write(env_path, content)?;
    Ok(())
}

fn get_state_file_path() -> PathBuf {
    PathBuf::from("./deployer/deployment_state.json")
}

fn ensure_deployer_dir() -> Result<()> {
    let deployer_dir = Path::new("./deployer");
    if !deployer_dir.exists() {
        fs::create_dir_all(deployer_dir)?;
    }
    Ok(())
}

fn create_initial_state() -> DeploymentState {
    let now = Utc::now().to_rfc3339();
    DeploymentState {
        started_at: now.clone(),
        last_updated: now,
        deployments: vec![
            ContractDeploymentState {
                order: 1,
                name: "Access Controller".to_string(),
                contract_name: "AccessController".to_string(),
                env_var: "ACCESS_CONTROLLER_CONTRACT_ID".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
            ContractDeploymentState {
                order: 2,
                name: "Asset Factory".to_string(),
                contract_name: "AssetFactory".to_string(),
                env_var: "ASSET_FACTORY".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
            ContractDeploymentState {
                order: 3,
                name: "Cradle Account Factory".to_string(),
                contract_name: "CradleAccountFactory".to_string(),
                env_var: "CRADLE_ACCOUNT_FACTORY_CONTRACT_ID".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
            ContractDeploymentState {
                order: 4,
                name: "Bridged Asset Issuer".to_string(),
                contract_name: "BridgedAssetIssuer".to_string(),
                env_var: "BRIDGED_ASSET_ISSUER_CONTRACT_ID".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
            ContractDeploymentState {
                order: 5,
                name: "Native Asset Issuer".to_string(),
                contract_name: "NativeAssetIssuer".to_string(),
                env_var: "NATIVE_ASSET_ISSUER_CONTRACT_ID".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
            ContractDeploymentState {
                order: 6,
                name: "Cradle Order Book Settler".to_string(),
                contract_name: "CradleOrderBookSettler".to_string(),
                env_var: "CRADLE_ORDER_BOOK_SETTLER_CONTRACT_ID".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
            ContractDeploymentState {
                order: 7,
                name: "Lending Pool Factory".to_string(),
                contract_name: "LendingPoolFactory".to_string(),
                env_var: "ASSET_LENDING_POOL_FACTORY".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
            ContractDeploymentState {
                order: 8,
                name: "Listings Factory".to_string(),
                contract_name: "CradleListingFactory".to_string(),
                env_var: "CRADLE_LISTING_FACTORY_CONTRACT_ID".to_string(),
                status: DeploymentStatus::Pending,
                contract_id: None,
            },
        ],
        tokens: vec![
            TokenState {
                name: "BASE_ASSET".to_string(),
                status: DeploymentStatus::Pending,
                id: None,
            },
            TokenState {
                name: "YIELD_ASSET".to_string(),
                status: DeploymentStatus::Pending,
                id: None,
            },
            TokenState {
                name: "RESERVE_ASSET_ID".to_string(),
                status: DeploymentStatus::Pending,
                id: None,
            },
        ],
    }
}

fn load_or_create_state() -> Result<DeploymentState> {
    ensure_deployer_dir()?;
    let state_path = get_state_file_path();

    if state_path.exists() {
        let content = fs::read_to_string(&state_path)?;
        let state = serde_json::from_str(&content)?;
        Ok(state)
    } else {
        Ok(create_initial_state())
    }
}

fn save_state(state: &DeploymentState) -> Result<()> {
    ensure_deployer_dir()?;
    let state_path = get_state_file_path();
    let mut updated_state = state.clone();
    updated_state.last_updated = Utc::now().to_rfc3339();
    let content = serde_json::to_string_pretty(&updated_state)?;
    fs::write(&state_path, content)?;
    Ok(())
}

fn update_deployment_status(
    state: &mut DeploymentState,
    env_var: &str,
    status: DeploymentStatus,
    contract_id: Option<String>,
) {
    for deployment in &mut state.deployments {
        if deployment.env_var == env_var {
            deployment.status = status;
            deployment.contract_id = contract_id;
            break;
        }
    }
}

fn update_token_status(
    state: &mut DeploymentState,
    token_name: &str,
    status: DeploymentStatus,
    id: Option<String>,
) {
    for token in &mut state.tokens {
        if token.name == token_name {
            token.status = status;
            token.id = id;
            break;
        }
    }
}

fn apply_state_to_env(state: &DeploymentState) {
    for deployment in &state.deployments {
        if let Some(contract_id) = &deployment.contract_id {
            unsafe {
                env::set_var(&deployment.env_var, contract_id);
            }
        }
    }

    for token in &state.tokens {
        if let Some(id) = &token.id {
            unsafe {
                env::set_var(&token.name, id);
            }
        }
    }
}

async fn deploy_contract_with_unlimited_retries(
    step: &DeploymentStep,
    deployed_ids: &mut HashMap<String, String>,
    state: &mut DeploymentState,
) -> Result<bool> {
    loop {
        if !Confirm::new()
            .with_prompt(format!("Deploy {} ({})?", step.name, step.contract_name))
            .interact()?
        {
            println!("  ⊘ Skipped {}", step.name);
            update_deployment_status(state, &step.env_var, DeploymentStatus::Failed, None);
            save_state(state)?;
            return Ok(false);
        }

        println!("  ⏳ Loading contract...");
        match step.contract_name.as_str() {
            "BridgedAssetIssuer" | "NativeAssetIssuer" => {
                let treasury_address: String =
                    Input::new().with_prompt("TREASURY_ADDRESS").interact()?;

                unsafe {
                    env::set_var("TREASURY_ADDRESS", treasury_address);
                }
            }
            _ => {
                // do nothing
            }
        }
        match Contract::load_contract_from_file(step.contract_name.clone()) {
            Ok(mut contract) => {
                println!("  ⏳ Deploying contract...");
                match contract.deploy_contract().await {
                    Ok(contract_id) => {
                        let contract_id_str = contract_id.to_string();
                        println!("  ✓ Deployment successful!");
                        println!("  📋 Contract ID: {}", contract_id_str);
                        unsafe {
                            env::set_var(step.env_var.clone(), contract_id_str.clone());
                        }
                        deployed_ids.insert(step.env_var.clone(), contract_id_str);
                        update_deployment_status(
                            state,
                            &step.env_var,
                            DeploymentStatus::Completed,
                            Some(contract_id.to_string()),
                        );
                        save_state(state)?;
                        return Ok(true);
                    }
                    Err(e) => {
                        println!("  ✗ Deployment failed: {}", e);
                        if !Confirm::new().with_prompt("Retry deployment?").interact()? {
                            println!("  Skipping to next step...");
                            update_deployment_status(
                                state,
                                &step.env_var,
                                DeploymentStatus::Failed,
                                None,
                            );
                            save_state(state)?;
                            return Ok(false);
                        }
                        // Loop continues for retry
                    }
                }
            }
            Err(e) => {
                println!("  ✗ Failed to load contract: {}", e);
                if !Confirm::new().with_prompt("Retry deployment?").interact()? {
                    println!("  Skipping to next step...");
                    update_deployment_status(state, &step.env_var, DeploymentStatus::Failed, None);
                    save_state(state)?;
                    return Ok(false);
                }
                // Loop continues for retry
            }
        }
    }
}
