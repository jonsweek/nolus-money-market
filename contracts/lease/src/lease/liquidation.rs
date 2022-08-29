use cosmwasm_std::Addr;

use finance::{
    coin::Coin,
    currency::{
        Currency,
        SymbolOwned
    },
    percent::Percent,
};

pub enum LiquidationStatus<Lpn>
where
    Lpn: Currency,
{
    None,
    FirstWarning(WarningAndLiquidationInfo),
    SecondWarning(WarningAndLiquidationInfo),
    ThirdWarning(WarningAndLiquidationInfo),
    PartialLiquidation(WarningAndLiquidationInfo, Coin<Lpn>),
    FullLiquidation(WarningAndLiquidationInfo, Coin<Lpn>),
}

pub struct WarningAndLiquidationInfo {
    pub customer: Addr,
    pub ltv: Percent,
    pub ltv_healthy: Percent,
    pub lease_asset: SymbolOwned,
}
