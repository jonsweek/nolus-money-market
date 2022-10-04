use finance::{
    currency::{AnyVisitor, Currency, Group, Member, Symbol},
    error::Error,
};

#[cfg(feature = "testing")]
use crate::test::{TestCurrencyA, TestCurrencyB, TestCurrencyC, TestCurrencyD};
use crate::{
    lease::{Atom, Osmo},
    lpn::Usdc,
    native::Nls,
};

impl Member<PaymentGroup> for Usdc {}
impl Member<PaymentGroup> for Osmo {}
impl Member<PaymentGroup> for Atom {}
impl Member<PaymentGroup> for Nls {}

pub struct PaymentGroup {}

const DESCR: &str = "payment";

impl Group for PaymentGroup {
    type ResolveError = Error;

    fn resolve<V>(symbol: Symbol, visitor: V) -> Result<V::Output, V::Error>
    where
        V: AnyVisitor<Self>,
        Self::ResolveError: Into<V::Error>,
    {
        match symbol {
            Usdc::SYMBOL => visitor.on::<Usdc>(),
            Osmo::SYMBOL => visitor.on::<Osmo>(),
            Atom::SYMBOL => visitor.on::<Atom>(),
            Nls::SYMBOL => visitor.on::<Nls>(),
            #[cfg(feature = "testing")]
            TestCurrencyA::SYMBOL => visitor.on::<TestCurrencyA>(),
            #[cfg(feature = "testing")]
            TestCurrencyB::SYMBOL => visitor.on::<TestCurrencyB>(),
            #[cfg(feature = "testing")]
            TestCurrencyC::SYMBOL => visitor.on::<TestCurrencyC>(),
            #[cfg(feature = "testing")]
            TestCurrencyD::SYMBOL => visitor.on::<TestCurrencyD>(),
            _ => Err(Error::NotInCurrencyGroup(symbol.into(), DESCR.into()).into()),
        }
    }
}
