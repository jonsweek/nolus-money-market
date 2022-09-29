use cosmwasm_std::{Addr, DepsMut, Env, Response};
use serde::{de::DeserializeOwned, Serialize};

use currency::payment::PaymentGroup;
use finance::currency::{visit_any, AnyVisitor, Currency};

use crate::{
    error::ContractError,
    msg::ExecuteMsg,
    state::{supported_pairs::SupportedPairs, Config},
};

use super::feed::try_feed_prices;

pub struct ExecWithOracleBase<'a> {
    deps: DepsMut<'a>,
    env: Env,
    msg: ExecuteMsg,
    sender: Addr,
}

impl<'a> ExecWithOracleBase<'a> {
    pub fn cmd(
        deps: DepsMut<'a>,
        env: Env,
        msg: ExecuteMsg,
        sender: Addr,
    ) -> Result<Response, ContractError> {
        let visitor = Self {
            deps,
            env,
            msg,
            sender,
        };

        let config = Config::load(visitor.deps.storage)?;
        visit_any(&config.base_asset, visitor)
    }
}

impl<'a> AnyVisitor<PaymentGroup> for ExecWithOracleBase<'a> {
    type Output = Response;
    type Error = ContractError;

    fn on<OracleBase>(self) -> Result<Self::Output, Self::Error>
    where
        OracleBase: 'static + Currency + DeserializeOwned + Serialize,
    {
        match self.msg {
            ExecuteMsg::CurrencyPaths { paths } => {
                let config = Config::load(self.deps.storage)?;
                if self.sender != config.owner {
                    return Err(ContractError::Unauthorized {});
                }
                SupportedPairs::<OracleBase>::new(paths)?.save(self.deps.storage)?;
                Ok(Response::default())
            }
            ExecuteMsg::FeedPrices { prices } => try_feed_prices::<OracleBase>(
                self.deps.storage,
                self.env.block.time,
                self.sender,
                prices,
            ),
            _ => {
                unreachable!()
            }
        }
    }
}