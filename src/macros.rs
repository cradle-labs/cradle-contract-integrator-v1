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

