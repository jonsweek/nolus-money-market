use currency::payment::PaymentGroup;
use finance::{currency::SymbolOwned, duration::Duration, price::dto::PriceDTO};
use sdk::{
    cosmwasm_std::{Addr, StdResult, Storage, Timestamp},
    cw_storage_plus::Map,
};

use crate::{
    error::PriceFeedsError,
    feed::{Observation, PriceFeed},
};

#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    price_feed_period: Duration,
    required_feeders_cnt: usize,
    block_time: Timestamp,
}

impl Parameters {
    pub fn new(
        price_feed_period: Duration,
        required_feeders_cnt: usize,
        block_time: Timestamp,
    ) -> Self {
        Parameters {
            price_feed_period,
            required_feeders_cnt,
            block_time,
        }
    }
    pub fn block_time(&self) -> Timestamp {
        self.block_time
    }
    pub fn feeders(&self) -> usize {
        self.required_feeders_cnt
    }
    pub fn period(&self) -> Duration {
        self.price_feed_period
    }
}

type DenomResolutionPath = Vec<PriceDTO>;
pub struct PriceFeeds<'m>(Map<'m, (SymbolOwned, SymbolOwned), PriceFeed>);

impl<'m> PriceFeeds<'m> {
    pub const fn new(namespace: &'m str) -> PriceFeeds {
        PriceFeeds(Map::new(namespace))
    }

    pub fn price(
        &self,
        storage: &dyn Storage,
        parameters: Parameters,
        path: Vec<SymbolOwned>,
    ) -> Result<PriceDTO, PriceFeedsError> {
        let mut resolution_path = DenomResolutionPath::new();

        if let Some((first, elements)) = path.split_first() {
            let mut base = first;
            for quote in elements {
                let price_dto =
                    self.load(storage, base.to_string(), quote.to_string(), parameters)?;
                base = quote;
                resolution_path.push(price_dto);
            }
        }
        PriceFeeds::calculate_price(&resolution_path)
    }

    pub fn load(
        &self,
        storage: &dyn Storage,
        base: SymbolOwned,
        quote: SymbolOwned,
        parameters: Parameters,
    ) -> Result<PriceDTO, PriceFeedsError> {
        match self.0.may_load(storage, (base, quote))? {
            Some(feed) => Ok(feed.get_price(parameters)?.price()),
            None => Err(PriceFeedsError::NoPrice()),
        }
    }
    // TODO remove move price calculation to the finance library
    fn calculate_price(resolution_path: &DenomResolutionPath) -> Result<PriceDTO, PriceFeedsError> {
        if let Some((first, rest)) = resolution_path.split_first() {
            rest.iter()
                .fold(Ok(first.to_owned()), |result_c1, c2| {
                    result_c1.and_then(|c1| c1.multiply::<PaymentGroup>(c2))
                })
                .map_err(|e| e.into())
        } else {
            Err(PriceFeedsError::NoPrice {})
        }
    }

    pub fn feed(
        &self,
        storage: &mut dyn Storage,
        current_block_time: Timestamp,
        sender_raw: &Addr,
        prices: Vec<PriceDTO>,
        price_feed_period: Duration,
    ) -> Result<(), PriceFeedsError> {
        for price_dto in prices {
            let update_market_price = |old: Option<PriceFeed>| -> StdResult<PriceFeed> {
                let new_feed =
                    Observation::new(sender_raw.clone(), current_block_time, price_dto.clone());
                match old {
                    Some(mut feed) => {
                        feed.update(new_feed, price_feed_period);
                        Ok(feed)
                    }
                    None => Ok(PriceFeed::new(new_feed)),
                }
            };

            self.0.update(
                storage,
                (
                    price_dto.base().ticker().to_string(),
                    price_dto.quote().ticker().to_string(),
                ),
                update_market_price,
            )?;
        }

        Ok(())
    }
}
