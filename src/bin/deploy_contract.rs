use anyhow::Result;
use clap::Parser;
use contract_integrator::utils::contract::Contract;

#[derive(Parser)]
struct ContractDeployer {
    #[clap(long, env)]
    pub deploy_contract_name: String,
}

#[tokio::main]
async fn main()->Result<()> {
    dotenv::dotenv()?;
    let deployment_args = ContractDeployer::parse();
    let contract_name = deployment_args.deploy_contract_name;

    let mut contract_to_deploy = Contract::load_contract_from_file(contract_name)?;

    contract_to_deploy.deploy_contract().await?;

    Ok(())
}