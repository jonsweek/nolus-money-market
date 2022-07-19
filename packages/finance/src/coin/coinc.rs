use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    currency::{Currency, SymbolOwned},
    error::Error,
};

use super::Coin;

/// A type designed to be used in the init, execute and query incoming messages.
/// It is a non-currency-parameterized version of finance::coin::Coin<C> with
/// the same representation on the wire. The aim is to use it everywhere the cosmwasm
/// framework does not support type parameterization.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoinDTO {
    amount: u128,
    symbol: SymbolOwned,
}

impl<C> TryFrom<CoinDTO> for Coin<C>
where
    C: Currency,
{
    type Error = Error;

    fn try_from(coin: CoinDTO) -> Result<Self, Self::Error> {
        if C::SYMBOL == coin.symbol {
            Ok(Self::new(coin.amount))
        } else {
            Err(Error::UnexpectedCurrency(coin.symbol, C::SYMBOL.into()))
        }
    }
}

impl<C> From<Coin<C>> for CoinDTO
where
    C: Currency,
{
    fn from(coin: Coin<C>) -> Self {
        Self {
            amount: coin.amount,
            symbol: C::SYMBOL.into(),
        }
    }
}

#[cfg(feature = "testing")]
pub fn funds<C>(amount: u128) -> CoinDTO
where
    C: Currency,
{
    Coin::<C>::new(amount).into()
}

#[cfg(test)]
mod test {
    use cosmwasm_std::to_vec;

    use crate::{currency::{Currency, SymbolStatic}, coin::{Coin, CoinDTO}};

    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct MyTestCurrency;
    impl Currency for MyTestCurrency {
        const SYMBOL: SymbolStatic = "qwerty";
    }

    #[test]
    fn same_representation() {
        let coin = Coin::<MyTestCurrency>::new(4215);
        assert_eq!(to_vec(&coin), to_vec(&CoinDTO::from(coin)));
    }
}