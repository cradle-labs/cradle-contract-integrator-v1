use anyhow::Result;
use dialoguer::{Confirm, Input};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use hedera::ContractId;
use contract_integrator::utils::contract::Contract;
use contract_integrator::utils::functions::asset_factory::{
    AssetFactoryFunctionInput, AssetFactoryFunctionOutput, CreateAssetArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[derive(Debug, Clone)]
struct DeploymentStep {
    name: String,
    contract_name: String,
    env_var: String,
    order: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

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
    ];

    println!("\n╔════════════════════════════════════════╗");
    println!("║   Contract Deployment Orchestrator    ║");
    println!("║         Interactive Deployer          ║");
    println!("╚════════════════════════════════════════╝\n");

    // Show deployment plan
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
    println!();

    // Confirm start
    if !Confirm::new()
        .with_prompt("Ready to begin contract deployments?")
        .interact()?
    {
        println!("Deployment cancelled.");
        return Ok(());
    }

    // Store deployed contract IDs
    let mut deployed_ids: HashMap<String, String> = HashMap::new();

    // Deploy contracts 1-6
    for (_idx, step) in steps.iter().take(6).enumerate() {
        // After Asset Factory (step 2), create tokens
        if step.order == 2 {
            println!(
                "\n┌─ Step {} of 7 ─────────────────────────────────┐",
                step.order
            );
            println!("│ Deploying: {:<35} │", step.name);
            println!("└───────────────────────────────────────────────┘");

            // Deploy Asset Factory first
            if !Confirm::new()
                .with_prompt(format!(
                    "Deploy {} ({})?",
                    step.name, step.contract_name
                ))
                .interact()?
            {
                println!("  ⊘ Skipped {}", step.name);
                continue;
            }

            println!("  ⏳ Loading contract...");
            match Contract::load_contract_from_file(step.contract_name.clone()) {
                Ok(mut contract) => {
                    println!("  ⏳ Deploying contract...");
                    match contract.deploy_contract().await {
                        Ok(contract_id) => {
                            let contract_id_str = contract_id.to_string();
                            println!("  ✓ Deployment successful!");
                            println!("  📋 Contract ID: {}", contract_id_str);
                            deployed_ids.insert(step.env_var.clone(), contract_id_str);

                            // Now create tokens immediately after Asset Factory
                            println!(
                                "\n┌─ Step 2.5 of 7 ───────────────────────────────┐"
                            );
                            println!(
                                "│ Creating Base Asset & Yield Asset Tokens      │"
                            );
                            println!("└───────────────────────────────────────────────┘");

                            match create_tokens(&deployed_ids).await {
                                Ok((base_id, yield_id)) => {
                                    // Store token IDs - we'll use transaction IDs
                                    deployed_ids.insert("BASE_ASSET".to_string(), base_id);
                                    deployed_ids.insert("YIELD_ASSET".to_string(), yield_id);
                                }
                                Err(e) => {
                                    println!("  ✗ Token creation failed: {}", e);
                                    if Confirm::new()
                                        .with_prompt("Retry token creation?")
                                        .interact()?
                                    {
                                        match create_tokens(&deployed_ids).await {
                                            Ok((base_id, yield_id)) => {
                                                println!("  ✓ Tokens created on retry!");
                                                deployed_ids.insert("BASE_ASSET".to_string(), base_id);
                                                deployed_ids.insert("YIELD_ASSET".to_string(), yield_id);
                                            }
                                            Err(retry_err) => {
                                                println!("  ✗ Retry failed: {}", retry_err);
                                                println!("  Continuing without tokens...");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("  ✗ Deployment failed: {}", e);
                            if Confirm::new()
                                .with_prompt("Retry deployment?")
                                .interact()?
                            {
                                match Contract::load_contract_from_file(step.contract_name.clone())
                                {
                                    Ok(mut retry_contract) => {
                                        match retry_contract.deploy_contract().await {
                                            Ok(contract_id) => {
                                                let contract_id_str = contract_id.to_string();
                                                println!("  ✓ Deployment successful on retry!");
                                                println!("  📋 Contract ID: {}", contract_id_str);
                                                deployed_ids.insert(
                                                    step.env_var.clone(),
                                                    contract_id_str,
                                                );
                                            }
                                            Err(retry_err) => {
                                                println!("  ✗ Retry failed: {}", retry_err);
                                                println!("  Continuing to next step...");
                                            }
                                        }
                                    }
                                    Err(load_err) => {
                                        println!("  ✗ Failed to load contract: {}", load_err);
                                        println!("  Continuing to next step...");
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("  ✗ Failed to load contract: {}", e);
                    println!("  Continuing to next step...");
                }
            }
        } else {
            // Standard deployment for other steps
            println!(
                "\n┌─ Step {} of 7 ─────────────────────────────────┐",
                step.order
            );
            println!("│ Deploying: {:<35} │", step.name);
            println!("└───────────────────────────────────────────────┘");

            // Get user confirmation
            if !Confirm::new()
                .with_prompt(format!(
                    "Deploy {} ({})?",
                    step.name, step.contract_name
                ))
                .interact()?
            {
                println!("  ⊘ Skipped {}", step.name);
                continue;
            }

            // Load and deploy contract
            println!("  ⏳ Loading contract...");
            match Contract::load_contract_from_file(step.contract_name.clone()) {
                Ok(mut contract) => {
                    println!("  ⏳ Deploying contract...");
                    match contract.deploy_contract().await {
                        Ok(contract_id) => {
                            let contract_id_str = contract_id.to_string();
                            println!("  ✓ Deployment successful!");
                            println!("  📋 Contract ID: {}", contract_id_str);
                            deployed_ids.insert(step.env_var.clone(), contract_id_str);
                        }
                        Err(e) => {
                            println!("  ✗ Deployment failed: {}", e);
                            if Confirm::new()
                                .with_prompt("Retry deployment?")
                                .interact()?
                            {
                                // Retry this step
                                match Contract::load_contract_from_file(step.contract_name.clone())
                                {
                                    Ok(mut retry_contract) => {
                                        match retry_contract.deploy_contract().await {
                                            Ok(contract_id) => {
                                                let contract_id_str = contract_id.to_string();
                                                println!("  ✓ Deployment successful on retry!");
                                                println!("  📋 Contract ID: {}", contract_id_str);
                                                deployed_ids.insert(
                                                    step.env_var.clone(),
                                                    contract_id_str,
                                                );
                                            }
                                            Err(retry_err) => {
                                                println!("  ✗ Retry failed: {}", retry_err);
                                                println!("  Continuing to next step...");
                                            }
                                        }
                                    }
                                    Err(load_err) => {
                                        println!("  ✗ Failed to load contract: {}", load_err);
                                        println!("  Continuing to next step...");
                                    }
                                }
                            } else {
                                println!("  Continuing to next step...");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("  ✗ Failed to load contract: {}", e);
                    println!("  Continuing to next step...");
                }
            }
        }
    }


    // Deploy Lending Pool Factory (Step 7)
    println!(
        "\n┌─ Step 7 of 7 ─────────────────────────────────┐"
    );
    println!(
        "│ Deploying: Lending Pool Factory              │"
    );
    println!("└───────────────────────────────────────────────┘");

    if !Confirm::new()
        .with_prompt("Deploy Lending Pool Factory?")
        .interact()?
    {
        println!("  ⊘ Skipped Lending Pool Factory");
    } else {
        println!("  ⏳ Loading contract...");
        match Contract::load_contract_from_file("LendingPoolFactory".to_string()) {
            Ok(mut contract) => {
                println!("  ⏳ Deploying contract...");
                match contract.deploy_contract().await {
                    Ok(contract_id) => {
                        let contract_id_str = contract_id.to_string();
                        println!("  ✓ Deployment successful!");
                        println!("  📋 Contract ID: {}", contract_id_str);
                        deployed_ids.insert(
                            "ASSET_LENDING_POOL_FACTORY".to_string(),
                            contract_id_str,
                        );
                    }
                    Err(e) => {
                        println!("  ✗ Deployment failed: {}", e);
                        if Confirm::new()
                            .with_prompt("Retry deployment?")
                            .interact()?
                        {
                            match Contract::load_contract_from_file("LendingPoolFactory".to_string())
                            {
                                Ok(mut retry_contract) => {
                                    match retry_contract.deploy_contract().await {
                                        Ok(contract_id) => {
                                            let contract_id_str = contract_id.to_string();
                                            println!("  ✓ Deployment successful on retry!");
                                            println!("  📋 Contract ID: {}", contract_id_str);
                                            deployed_ids.insert(
                                                "ASSET_LENDING_POOL_FACTORY".to_string(),
                                                contract_id_str,
                                            );
                                        }
                                        Err(retry_err) => {
                                            println!("  ✗ Retry failed: {}", retry_err);
                                        }
                                    }
                                }
                                Err(load_err) => {
                                    println!("  ✗ Failed to load contract: {}", load_err);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("  ✗ Failed to load contract: {}", e);
            }
        }
    }

    // Display summary
    println!(
        "\n╔════════════════════════════════════════╗"
    );
    println!(
        "║        Deployment Summary              ║"
    );
    println!(
        "╚════════════════════════════════════════╝\n"
    );

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

async fn create_tokens(
    deployed_ids: &HashMap<String, String>,
) -> Result<(String, String)> {
    let mut wallet = ActionWallet::from_env();

    // Get Access Controller contract ID from deployed contracts or env
    let acl_contract_str = deployed_ids
        .get("ACCESS_CONTROLLER_CONTRACT_ID")
        .cloned()
        .unwrap_or_else(|| env::var("ACCESS_CONTROLLER_CONTRACT_ID").unwrap_or_default());

    // Parse string to ContractId and convert to Solidity address
    let acl_contract_id = ContractId::from_str(&acl_contract_str)?;
    let acl_contract_solidity = acl_contract_id.to_solidity_address()?;

    let allow_list: u64 = 1;

    // Create Base Asset
    println!("┌─ Creating Base Asset Token ────────────────┐");
    let base_name: String = Input::new()
        .with_prompt("Base Asset Name")
        .interact()?;
    let base_symbol: String = Input::new()
        .with_prompt("Base Asset Symbol")
        .interact()?;

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

    let base_asset_id = if let ContractCallOutput::AssetFactory(
        AssetFactoryFunctionOutput::CreateAsset(output),
    ) = base_asset_result
    {
        println!("  ✓ Base Asset created!");
        println!("  📋 Transaction ID: {}", output.transaction_id);
        output.transaction_id
    } else {
        anyhow::bail!("Unexpected response from Asset Factory")
    };

    println!("└────────────────────────────────────────────┘\n");

    // Create Yield Asset
    println!("┌─ Creating Yield Asset Token ───────────────┐");
    let yield_name: String = Input::new()
        .with_prompt("Yield Asset Name")
        .interact()?;
    let yield_symbol: String = Input::new()
        .with_prompt("Yield Asset Symbol")
        .interact()?;

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

    let yield_asset_id = if let ContractCallOutput::AssetFactory(
        AssetFactoryFunctionOutput::CreateAsset(output),
    ) = yield_asset_result
    {
        println!("  ✓ Yield Asset created!");
        println!("  📋 Transaction ID: {}", output.transaction_id);
        output.transaction_id
    } else {
        anyhow::bail!("Unexpected response from Asset Factory")
    };

    println!("└────────────────────────────────────────────┘\n");

    println!("✓ Tokens created successfully:");
    println!("  Base Asset: {}", base_asset_id);
    println!("  Yield Asset: {}", yield_asset_id);

    Ok((base_asset_id, yield_asset_id))
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
