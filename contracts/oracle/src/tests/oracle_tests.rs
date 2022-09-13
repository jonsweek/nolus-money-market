use std::collections::HashSet;

use crate::contract::{execute, query};
use crate::msg::{ExecuteMsg, PricesResponse, QueryMsg};
use crate::tests::{dummy_default_instantiate_msg, setup_test};
use crate::ContractError;

use cosmwasm_std::from_binary;
use cosmwasm_std::testing::{mock_env, mock_info};
use finance::coin::Coin;
use finance::currency::{
    Currency, SymbolStatic, TestCurrencyA, TestCurrencyB, TestCurrencyC, Usdc,
};
use finance::price::{self, PriceDTO};

use super::dummy_feed_prices_msg;

#[test]
fn feed_prices_unknown_feeder() {
    let (mut deps, _) = setup_test(dummy_default_instantiate_msg());

    let msg = dummy_feed_prices_msg();
    let info = mock_info("test", &[]);

    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(ContractError::UnknownFeeder {}, err)
}

#[test]
fn feed_direct_price() {
    let (mut deps, info) = setup_test(dummy_default_instantiate_msg());

    let expected_price = PriceDTO::try_from(
        price::total_of(Coin::<TestCurrencyA>::new(10)).is(Coin::<Usdc>::new(120)),
    )
    .unwrap();

    // Feed direct price TestCurrencyA/OracleBaseAsset
    let msg = ExecuteMsg::FeedPrices {
        prices: vec![expected_price.clone()],
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // query price for TestCurrencyA
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::PriceFor {
            currencies: HashSet::from([TestCurrencyA::SYMBOL.to_string()]),
        },
    )
    .unwrap();
    let value: PricesResponse = from_binary(&res).unwrap();
    assert_eq!(expected_price, value.prices.first().unwrap().to_owned());
}

#[test]
fn feed_indirect_price() {
    let (mut deps, info) = setup_test(dummy_default_instantiate_msg());

    let price_a_to_b = PriceDTO::try_from(
        price::total_of(Coin::<TestCurrencyA>::new(10)).is(Coin::<TestCurrencyB>::new(120)),
    )
    .unwrap();
    let price_b_to_c = PriceDTO::try_from(
        price::total_of(Coin::<TestCurrencyB>::new(10)).is(Coin::<TestCurrencyC>::new(5)),
    )
    .unwrap();
    let price_c_to_usdc = PriceDTO::try_from(
        price::total_of(Coin::<TestCurrencyC>::new(10)).is(Coin::<Usdc>::new(5)),
    )
    .unwrap();

    // Feed direct price TestCurrencyA/OracleBaseAsset
    let msg = ExecuteMsg::FeedPrices {
        prices: vec![price_a_to_b, price_b_to_c, price_c_to_usdc],
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // query price for TestCurrencyA
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Price {
            currency: TestCurrencyA::SYMBOL.to_string(),
        },
    )
    .unwrap();

    let expected_price =
        PriceDTO::try_from(price::total_of(Coin::<TestCurrencyA>::new(1)).is(Coin::<Usdc>::new(6)))
            .unwrap();
    let value: PricesResponse = from_binary(&res).unwrap();
    assert_eq!(expected_price, value.prices.first().unwrap().to_owned());
}

#[test]
#[should_panic(expected = "Unsupported denom")]
fn query_prices_unsuppoted_denom() {
    let (deps, _) = setup_test(dummy_default_instantiate_msg());

    // query for unsupported denom should fail
    query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::PriceFor {
            currencies: HashSet::from(["dummy".to_string()]),
        },
    )
    .unwrap();
}

#[test]
fn feed_prices_unsupported_pairs() {
    let (mut deps, info) = setup_test(dummy_default_instantiate_msg());
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
    pub struct X;
    impl Currency for X {
        const SYMBOL: SymbolStatic = "X";
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
    pub struct C;
    impl Currency for C {
        const SYMBOL: SymbolStatic = "C";
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
    pub struct D;
    impl Currency for D {
        const SYMBOL: SymbolStatic = "D";
    }

    let prices_map = vec![
        PriceDTO::try_from(price::total_of(Coin::<X>::new(10)).is(Coin::<C>::new(12))).unwrap(),
        PriceDTO::try_from(price::total_of(Coin::<X>::new(10)).is(Coin::<D>::new(22))).unwrap(),
    ];

    let msg = ExecuteMsg::FeedPrices { prices: prices_map };
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(ContractError::UnsupportedDenomPairs {}, err);
}
