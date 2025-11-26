#[macro_export]
macro_rules! id_to_address {
    ($id_str: expr) => {{
        let id = ContractId::from_str(&$id_str)?;
        let address = id.to_solidity_address()?;
        address
    }};
    ($id_str: literal) => {{
        let id = ContractId::from_str(id_str)?;
        let address = id.to_solidity_address()?;
        address
    }};
}

#[macro_export]
macro_rules! id_to_evm_address {
    ($contract_id: literal) => {
        let evm_address = get_contract_address($contract_id)?;
        evm_address
    };
}
