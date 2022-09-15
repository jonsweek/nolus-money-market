use std::collections::HashSet;

use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use finance::{currency::SymbolOwned, price::dto::PriceDTO};
use marketprice::alarms::Alarm;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub base_asset: String,
    pub price_feed_period_secs: u32,
    pub feeders_percentage_needed: u8,
    pub supported_denom_pairs: Vec<(SymbolOwned, SymbolOwned)>,
    pub timealarms_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterFeeder {
        feeder_address: String,
    },
    FeedPrices {
        prices: Vec<PriceDTO>,
    },
    Config {
        price_feed_period_secs: u32,
        feeders_percentage_needed: u8,
    },
    SupportedDenomPairs {
        pairs: Vec<(SymbolOwned, SymbolOwned)>,
    },
    AddPriceAlarm {
        alarm: Alarm,
    },
    RemovePriceAlarm {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // returns the contract configuration
    Config {},
    // returns all registered feeders
    Feeders {},
    // check if an address belongs to a registered feeder
    IsFeeder { address: Addr },
    // returns the price of the denom against the base asset
    PriceFor { currencies: HashSet<SymbolOwned> },
    Price { currency: SymbolOwned },
    // returns a list of supported denom pairs
    SupportedDenomPairs {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ConfigResponse {
    pub base_asset: SymbolOwned,
    pub price_feed_period_secs: u32,
    pub feeders_percentage_needed: u8,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PricesResponse {
    pub prices: Vec<PriceDTO>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PriceResponse {
    pub price: PriceDTO,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteAlarmMsg {
    PriceAlarm(),
}
