use cosmwasm_std::StdError;
use thiserror::Error;

use finance::{currency::Currency, error::Error as FinanceError};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("[Platform] {0}")]
    FinanceError(#[from] FinanceError),

    #[error("[Platform] Expecting funds of {0} but found none")]
    NoFunds(String),

    #[error("[Platform] Expecting funds of {0} but found extra ones")]
    UnexpectedFunds(String),

    #[error("[Platform] [Std] {0}")]
    CosmWasmError(#[from] StdError),
}

impl Error {
    pub fn no_funds<C>() -> Self
    where
        C: Currency,
    {
        Self::NoFunds(C::SYMBOL.into())
    }

    pub fn unexpected_funds<C>() -> Self
    where
        C: Currency,
    {
        Self::UnexpectedFunds(C::SYMBOL.into())
    }
}

pub type Result<T> = core::result::Result<T, Error>;
