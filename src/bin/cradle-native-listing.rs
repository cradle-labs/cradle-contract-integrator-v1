use contract_integrator::{
    utils::functions::{
        ContractCallInput, ContractCallOutput, WithContractId,
        cradle_native_listing::{
            CradleNativeListingFunctionsInput, CradleNativeListingFunctionsOutput, ListingStatus,
            PurchaseInputArgs, ReturnAssetInputArgs, WithdrawToBeneficiaryInputArgs,
        },
    },
    wallet::wallet::ActionWallet,
};
use dialoguer::{Input, Select};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let mut wallet = ActionWallet::from_env();

    let function_selection = Select::new()
        .with_prompt("Select an action:")
        .item("Update Listing Status")
        .item("Purchase")
        .item("Return Asset")
        .item("WithdrawToBeneficiary")
        .item("GetListingStats")
        .item("GetFee")
        .interact()?;

    let contract_id: String = Input::new().with_prompt("Contract ID").interact()?;

    let input = {
        match function_selection {
            0 => {
                let new_status = Select::new()
                    .with_prompt("Select new listing Status")
                    .item("Pending")
                    .item("Open")
                    .item("Closed")
                    .item("Paused")
                    .item("Cancelled")
                    .interact()?;

                ContractCallInput::CradleNativeListing(
                    CradleNativeListingFunctionsInput::UpdateListingStatus(WithContractId {
                        contract_id,
                        rest: Some(ListingStatus::from(new_status as u8)),
                    }),
                )
            }
            1 => {
                let account: String = Input::new().with_prompt("Account").interact()?;

                let amount: u64 = Input::new().with_prompt("Amount").interact()?;

                ContractCallInput::CradleNativeListing(CradleNativeListingFunctionsInput::Purchase(
                    WithContractId {
                        contract_id,
                        rest: Some(PurchaseInputArgs {
                            buyer: account,
                            amount,
                        }),
                    },
                ))
            }
            2 => {
                let account: String = Input::new().with_prompt("Account").interact()?;

                let amount: u64 = Input::new().with_prompt("Amount").interact()?;

                ContractCallInput::CradleNativeListing(
                    CradleNativeListingFunctionsInput::ReturnAsset(WithContractId {
                        contract_id,
                        rest: Some(ReturnAssetInputArgs { account, amount }),
                    }),
                )
            }
            3 => {
                let amount: u64 = Input::new().with_prompt("Amount").interact()?;

                ContractCallInput::CradleNativeListing(
                    CradleNativeListingFunctionsInput::WithdrawToBeneficiary(WithContractId {
                        contract_id,
                        rest: Some(WithdrawToBeneficiaryInputArgs { amount }),
                    }),
                )
            }
            4 => ContractCallInput::CradleNativeListing(
                CradleNativeListingFunctionsInput::GetListingStats(WithContractId {
                    contract_id,
                    rest: None,
                }),
            ),
            5 => {
                let amount: u64 = Input::new().with_prompt("Amount").interact()?;

                ContractCallInput::CradleNativeListing(CradleNativeListingFunctionsInput::GetFee(
                    WithContractId {
                        contract_id,
                        rest: Some(amount),
                    },
                ))
            }
            _ => {
                panic!("Invalid selection")
            }
        }
    };

    let res = wallet.execute(input).await?;

    match res {
        ContractCallOutput::CradleNativeListing(r) => match r {
            CradleNativeListingFunctionsOutput::UpdateListingStatus(d) => {
                println!("Transaction commited :: {:?}", d.transaction_id);
            }
            CradleNativeListingFunctionsOutput::Purchase(d) => {
                println!("Transaction commited :: {:?}", d.transaction_id);
                println!("Amount received {:?}", d.output.unwrap());
            }
            CradleNativeListingFunctionsOutput::ReturnAsset(d) => {
                println!("Transaction commited :: {:?}", d.transaction_id);
                println!("Amount received {:?}", d.output.unwrap());
            }
            CradleNativeListingFunctionsOutput::WithdrawToBeneficiary(d) => {
                println!("Transaction commited :: {:?}", d.transaction_id);
            }
            CradleNativeListingFunctionsOutput::GetListingStats(d) => {
                println!("Transaction commited :: {:?}", d.transaction_id);
                println!("Listing stats :: {:?}", d.output.unwrap());
            }
            CradleNativeListingFunctionsOutput::GetFee(d) => {
                println!("Transaction commited :: {:?} ", d.transaction_id);
                println!("Fee :: {:?}", d.output.unwrap());
            }
        },
        _ => panic!("invalid path :) "),
    }

    Ok(())
}
