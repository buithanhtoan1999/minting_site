use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("You not in whitelist")]
    WhitelistError {},
    
    #[error("InvalidUnitPrice")]
    InvalidUnitPrice {},

    #[error("MintingEnded")]
    MintingEnded {},

    #[error("You need to send right amount")]
    ZeroAmount {},

    #[error("You send wrong amount")]
    WrongAmount


}
