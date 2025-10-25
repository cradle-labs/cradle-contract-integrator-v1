use anyhow::Result;
use dialoguer::{Input, Select};
use contract_integrator::utils::functions::cradle_account_factory::{
    CradleAccountFactoryFunctionsInput, CradleAccountFactoryFunctionsOutput,
    CreateAccountInputArgs, CreateAccountForUserInputArgs, GetAccountByControllerInputArgs,
};
use contract_integrator::utils::functions::{ContractCallInput, ContractCallOutput};
use contract_integrator::wallet::wallet::ActionWallet;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let mut wallet = ActionWallet::from_env();

    // Select which function to call
    let function_selection = Select::new()
        .with_prompt("Select Cradle Account Factory Function")
        .items(&[
            "Create Account",
            "Create Account For User",
            "Get Account By Controller",
        ])
        .interact()?;

    let input = match function_selection {
        0 => {
            // CreateAccount
            let controller: String = Input::new()
                .with_prompt("Controller Name")
                .interact()?;
            let account_allow_list: String = Input::new()
                .with_prompt("Allow List Value")
                .interact()?;

            ContractCallInput::CradleAccountFactory(
                CradleAccountFactoryFunctionsInput::CreateAccount(CreateAccountInputArgs {
                    controller,
                    account_allow_list,
                }),
            )
        }
        1 => {
            // CreateAccountForUser
            let controller: String = Input::new()
                .with_prompt("Controller Name")
                .interact()?;
            let user: String = Input::new()
                .with_prompt("User Address")
                .interact()?;
            let account_allow_list: String = Input::new()
                .with_prompt("Allow List Value")
                .interact()?;

            ContractCallInput::CradleAccountFactory(
                CradleAccountFactoryFunctionsInput::CreateAccountForUser(
                    CreateAccountForUserInputArgs {
                        controller,
                        user,
                        account_allow_list,
                    },
                ),
            )
        }
        2 => {
            // GetAccountByController
            let controller: String = Input::new()
                .with_prompt("Controller Name")
                .interact()?;

            ContractCallInput::CradleAccountFactory(
                CradleAccountFactoryFunctionsInput::GetAccountByController(
                    GetAccountByControllerInputArgs { controller },
                ),
            )
        }
        _ => panic!("Invalid selection"),
    };

    // Execute the contract call
    let res = wallet.execute(input).await?;

    // Handle the response
    match res {
        ContractCallOutput::CradleAccountFactory(output) => {
            match output {
                CradleAccountFactoryFunctionsOutput::CreateAccount(result) => {
                    println!("✓ Account Created");
                    println!("Transaction ID: {}", result.transaction_id);
                    if let Some(info) = result.output {
                        println!("Account Address: {}", info.account_address);
                    }
                }
                CradleAccountFactoryFunctionsOutput::CreateAccountForUser(result) => {
                    println!("✓ Account Created For User");
                    println!("Transaction ID: {}", result.transaction_id);
                    if let Some(info) = result.output {
                        println!("Account Address: {}", info.account_address);
                    }
                }
                CradleAccountFactoryFunctionsOutput::GetAccountByController(result) => {
                    println!("✓ Account Retrieved");
                    if let Some(info) = result.output {
                        println!("Account Address: {}", info.account_address);
                    }
                }
            }
        }
        _ => println!("Unexpected contract response"),
    }

    Ok(())
}
