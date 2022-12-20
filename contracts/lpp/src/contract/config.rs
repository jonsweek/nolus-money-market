use cosmwasm_std::MessageInfo;
use finance::percent::Percent;
use platform::access_control::SingleUserAccess;
use sdk::{
    cosmwasm_ext::Response,
    cosmwasm_std::{Deps, DepsMut},
};

use crate::{error::ContractError, msg::QueryConfigResponse, state::Config};

pub fn try_update_parameters(
    deps: DepsMut,
    info: MessageInfo,
    base_interest_rate: Percent,
    utilization_optimal: Percent,
    addon_optimal_interest_rate: Percent,
) -> Result<Response, ContractError> {
    SingleUserAccess::load(deps.storage, crate::access_control::OWNER_NAMESPACE)?
        .check_access(&info.sender)?;

    let mut config = Config::load(deps.storage)?;

    config.update(
        deps.storage,
        base_interest_rate,
        utilization_optimal,
        addon_optimal_interest_rate,
    )?;

    Ok(Response::new().add_attribute("method", "try_update_parameters"))
}

pub fn query_config(deps: &Deps) -> Result<QueryConfigResponse, ContractError> {
    let config = Config::load(deps.storage)?;

    Ok(QueryConfigResponse {
        lpn_ticker: config.lpn_ticker,
        lease_code_id: config.lease_code_id,
        base_interest_rate: config.base_interest_rate,
        utilization_optimal: config.utilization_optimal,
        addon_optimal_interest_rate: config.addon_optimal_interest_rate,
    })
}
