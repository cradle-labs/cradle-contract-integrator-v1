use anyhow::Result;
use dialoguer::Input;
use contract_integrator::utils::functions::orderbook_settler::{
    OrderBookSettlerFunctionInput, OrderBookSettlerFunctionOutput, SettleOrderInputArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // SettleOrder
    let bidder: String = Input::new()
        .with_prompt("Bidder Address")
        .interact()?;
    let asker: String = Input::new()
        .with_prompt("Asker Address")
        .interact()?;
    let bid_asset: String = Input::new()
        .with_prompt("Bid Asset Address")
        .interact()?;
    let ask_asset: String = Input::new()
        .with_prompt("Ask Asset Address")
        .interact()?;
    let bid_asset_amount: String = Input::new()
        .with_prompt("Bid Asset Amount")
        .interact()?;
    let ask_asset_amount: String = Input::new()
        .with_prompt("Ask Asset Amount")
        .interact()?;

    let input = ContractCallInput::OrderBookSettler(OrderBookSettlerFunctionInput::SettleOrder(
        SettleOrderInputArgs {
            bidder,
            asker,
            bid_asset,
            ask_asset,
            bid_asset_amount,
            ask_asset_amount,
        },
    ));

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response
    match res {
        ContractCallOutput::OrderBookSettler(output) => {
            match output {
                OrderBookSettlerFunctionOutput::SettleOrder(result) => {
                    println!("✓ Order Settled");
                    println!("Transaction ID: {}", result.transaction_id);
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
