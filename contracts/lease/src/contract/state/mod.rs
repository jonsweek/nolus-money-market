use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use sdk::{
    cosmwasm_ext::Response as CwResponse,
    cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Reply},
    neutron_sdk::sudo::msg::SudoMsg,
};

use crate::{
    api::{ExecuteMsg, StateQuery, StateResponse},
    error::{ContractError as Err, ContractResult},
};

pub use self::{
    active::Active, buy_asset::BuyAsset, open_ica_account::OpenIcaAccount,
    request_loan::RequestLoan, transfer_out::TransferOut,
};

mod active;
mod buy_asset;
mod open_ica_account;
mod opening;
mod request_loan;
mod transfer_out;

type OpeningTransferOut = transfer_out::Controller<opening::transfer_out::TransferOut>;

#[enum_dispatch(Controller)]
#[derive(Serialize, Deserialize)]
pub enum State {
    RequestLoan,
    OpenIcaAccount,
    OpeningTransferOut,
    BuyAsset,
    Active,
}

pub struct Response {
    pub(super) cw_response: CwResponse,
    pub(super) next_state: State,
}

impl Response {
    pub fn from<R, S>(resp: R, next_state: S) -> Self
    where
        R: Into<CwResponse>,
        S: Into<State>,
    {
        Self {
            cw_response: resp.into(),
            next_state: next_state.into(),
        }
    }
}

#[enum_dispatch]
pub trait Controller
where
    Self: Sized,
{
    fn reply(self, _deps: &mut DepsMut, _env: Env, _msg: Reply) -> ContractResult<Response> {
        err("reply")
    }

    fn execute(
        self,
        _deps: &mut DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        err("execute")
    }

    fn query(self, _deps: Deps, _env: Env, _msg: StateQuery) -> ContractResult<StateResponse>;

    fn sudo(self, _deps: &mut DepsMut, _env: Env, _msg: SudoMsg) -> ContractResult<Response> {
        err("sudo")
    }
}

fn err<R>(op: &str) -> ContractResult<R> {
    Err(Err::unsupported_operation(op))
}
