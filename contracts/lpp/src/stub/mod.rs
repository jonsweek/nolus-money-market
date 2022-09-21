use std::{marker::PhantomData, result::Result as StdResult};

use cosmwasm_std::{Addr, QuerierWrapper};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use finance::currency::{visit_any, AnyVisitor, Currency, SymbolOwned};
use platform::batch::Batch;

use crate::{
    error::{ContractError, ContractResult},
    msg::{LppBalanceResponse, QueryConfigResponse, QueryMsg},
};

pub mod lender;

pub trait Lpp<Lpn>
where
    Self: Into<LppBatch<LppRef>>,
    Lpn: Currency,
{
    fn lpp_balance(&self) -> ContractResult<LppBalanceResponse<Lpn>>;
}

pub trait WithLpp {
    type Output;
    type Error;

    fn exec<C, L>(self, lpp: L) -> StdResult<Self::Output, Self::Error>
    where
        L: Lpp<C>,
        C: Currency + Serialize;

    fn unknown_lpn(self, symbol: SymbolOwned) -> StdResult<Self::Output, Self::Error>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LppRef {
    addr: Addr,
    currency: SymbolOwned,
}

impl LppRef {
    pub fn try_new(addr: Addr, querier: &QuerierWrapper) -> ContractResult<Self> {
        let resp: QueryConfigResponse =
            querier.query_wasm_smart(addr.clone(), &QueryMsg::Config())?;

        let currency = resp.lpn_symbol;

        Ok(Self { addr, currency })
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }

    pub fn execute<V, O, E>(self, cmd: V, querier: &QuerierWrapper) -> StdResult<O, E>
    where
        V: WithLpp<Output = O, Error = E>,
    {
        struct CurrencyVisitor<'a, V, O, E>
        where
            V: WithLpp<Output = O, Error = E>,
        {
            cmd: V,
            lpp_ref: LppRef,
            querier: &'a QuerierWrapper<'a>,
        }

        impl<'a, V, O, E> AnyVisitor for CurrencyVisitor<'a, V, O, E>
        where
            V: WithLpp<Output = O, Error = E>,
        {
            type Output = O;
            type Error = E;

            fn on<C>(self) -> StdResult<Self::Output, Self::Error>
            where
                C: Currency + Serialize + DeserializeOwned,
            {
                self.cmd.exec(self.lpp_ref.into_stub::<C>(self.querier))
            }

            fn on_unknown(self) -> StdResult<Self::Output, Self::Error> {
                self.cmd.unknown_lpn(self.lpp_ref.currency)
            }
        }

        visit_any(
            &self.currency.clone(),
            CurrencyVisitor {
                cmd,
                lpp_ref: self,
                querier,
            },
        )
    }

    fn into_stub<'a, C>(self, querier: &'a QuerierWrapper) -> LppStub<'a, C> {
        LppStub {
            lpp_ref: self,
            currency: PhantomData::<C>,
            querier,
            batch: Batch::default(),
        }
    }
}

#[cfg(feature = "testing")]
impl LppRef {
    pub fn unchecked<A, Lpn>(addr: A) -> Self
    where
        A: Into<String>,
        Lpn: Currency,
    {
        Self {
            addr: Addr::unchecked(addr),
            currency: Lpn::SYMBOL.into(),
        }
    }
}

struct LppStub<'a, C> {
    lpp_ref: LppRef,
    currency: PhantomData<C>,
    querier: &'a QuerierWrapper<'a>,
    batch: Batch,
}

impl<'a, C> LppStub<'a, C> {
    fn id(&self) -> Addr {
        self.lpp_ref.addr.clone()
    }
}

impl<'a, Lpn> Lpp<Lpn> for LppStub<'a, Lpn>
where
    Lpn: Currency + DeserializeOwned,
{
    fn lpp_balance(&self) -> ContractResult<LppBalanceResponse<Lpn>> {
        let msg = QueryMsg::LppBalance();
        self.querier
            .query_wasm_smart(self.id(), &msg)
            .map_err(ContractError::from)
    }
}

pub struct LppBatch<Ref> {
    pub lpp_ref: Ref,
    pub batch: Batch,
}

impl<'a, C> From<LppStub<'a, C>> for LppBatch<LppRef> {
    fn from(stub: LppStub<'a, C>) -> Self {
        Self {
            lpp_ref: stub.lpp_ref,
            batch: stub.batch,
        }
    }
}