#[cfg(feature = "cosmwasm-bindings")]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Api, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use crate::alarms::TimeAlarms;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddAlarm { time } => TimeAlarms::try_add(deps, info.sender, time),
        ExecuteMsg::Notify {} => TimeAlarms::try_notify(deps.storage, env.block.time),
    }
}

pub fn get_sender(api: &dyn Api, info: MessageInfo) -> StdResult<Addr> {
    api.addr_validate(info.sender.as_str())
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let res = match msg.result {
        cosmwasm_std::SubMsgResult::Ok(_) => {
            TimeAlarms::remove(deps.storage, msg.id)?;
            Response::new().add_attribute("alarm", "success")
        }
        cosmwasm_std::SubMsgResult::Err(err) => Response::new()
            .add_attribute("alarm", "error")
            .add_attribute("error", err),
    };
    Ok(res)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    use crate::msg::InstantiateMsg;

    use super::instantiate;

    #[test]
    fn proper_initialization() {
        let msg = InstantiateMsg {};
        let mut deps = mock_dependencies();
        let info = mock_info("CREATOR", &coins(1000, "token"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}