use anyhow::Result;
use contract_integrator::{
    utils::functions::{
        ContractCallInput, ContractCallOutput,
        listing_factory::{
            CradleListingFactoryFunctionsInput, CradleListingFactoryFunctionsOutput, CreateListing,
        },
    },
    wallet::wallet::ActionWallet,
};
use dialoguer::Input;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let mut wallet = ActionWallet::from_env();

    let input = {
        let fee_collector: String = Input::new().with_prompt("Fee collector").interact()?;
        let reserve_account: String = Input::new().with_prompt("Reserve Account").interact()?;
        let max_supply: u64 = Input::new().with_prompt("Max supply").interact()?;
        let listing_asset: String = Input::new().with_prompt("Listing Asset").interact()?;
        let purchase_asset: String = Input::new().with_prompt("Purchase Asset").interact()?;
        let purchase_price: u64 = Input::new().with_prompt("Purchase Price").interact()?;
        let beneficiary_address: String =
            Input::new().with_prompt("Beneficiary Address").interact()?;
        let shadow_asset: String = Input::new().with_prompt("Shadow Asset").interact()?;

        ContractCallInput::CradleListingFactory(CradleListingFactoryFunctionsInput::CreateListing(
            CreateListing {
                fee_collector_address: fee_collector,
                reserve_account,
                max_supply,
                listing_asset,
                purchase_asset,
                purchase_price,
                beneficiary_address,
                shadow_asset,
            },
        ))
    };

    let res = wallet.execute(input).await?;

    match res {
        ContractCallOutput::CradleListingFactory(
            CradleListingFactoryFunctionsOutput::CreateListing(output),
        ) => {
            println!("Transaction success :: {:?}", output.transaction_id);
            println!("New Listing created at :: {:?}", output.output.unwrap());
        }
        _ => {
            // println!("Transaction Failed")
        }
    }

    Ok(())
}
