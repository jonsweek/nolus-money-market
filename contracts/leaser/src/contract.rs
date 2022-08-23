#[cfg(feature = "cosmwasm-bindings")]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Api, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, Storage,
};
use cw2::set_contract_version;

use platform::cosmwasm_protobuf::from_instantiate_reply;

use crate::{
    cmd::Borrow,
    error::ContractError,
    leaser::Leaser,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{
        config::Config,
        leaser::Loans,
    },
};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config::new(info.sender, msg)?;
    config.store(deps.storage)?;

    Ok(Response::default())
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::OpenLease { currency } => Borrow::with(deps, info.funds, info.sender, currency),
        ExecuteMsg::Config {
            lease_interest_rate_margin,
            liability,
            repayment,
        } => Leaser::try_configure(deps, info, lease_interest_rate_margin, liability, repayment),
    }
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let res = match msg {
        QueryMsg::Config {} => to_binary(&Leaser::query_config(deps)?),
        QueryMsg::Quote { downpayment } => to_binary(&Leaser::query_quote(deps, downpayment)?),
        QueryMsg::Leases { owner } => to_binary(&Leaser::query_loans(deps, owner)?),
    };
    res.map_err(ContractError::from)
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match on_reply(deps.api, deps.storage, msg.clone()) {
        Ok(resp) => Ok(resp),
        Err(err) => {
            Loans::remove(deps.storage, msg.id);
            Err(ContractError::CustomError {
                val: err.to_string(),
            })
        }
    }
}

fn on_reply(
    api: &dyn Api,
    storage: &mut dyn Storage,
    msg: Reply,
) -> Result<Response, ContractError> {
    let contract_addr_raw = from_instantiate_reply::<()>(msg.clone())
        .map(|r| r.address)
        .map_err(|err| ContractError::ParseError {
            err: err.to_string(),
        })?;

    let contract_addr = api.addr_validate(&contract_addr_raw)?;

    Loans::save(storage, msg.id, contract_addr.clone())?;
    Ok(Response::new().add_attribute("lease_address", contract_addr))
}
