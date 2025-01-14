use serde::{Deserialize, Serialize};

use sdk::{
    cosmwasm_std::Addr,
    schemars::{self, JsonSchema},
};

use crate::state::reward_scale::RewardScale;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub cadence_hours: u16,
    pub lpp: Addr,
    pub oracle: Addr,
    pub timealarms: Addr,
    pub treasury: Addr,
    pub tvl_to_apr: RewardScale,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MigrateMsg {
    pub contract_owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Config { cadence_hours: u16 },
    TimeAlarm {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ConfigResponse {
    pub cadence_hours: u16,
}
