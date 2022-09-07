use cosmwasm_std::{wasm_execute, Addr};
use cw_multi_test::Executor;

use finance::{
    coin::Coin,
    currency::{Currency as CurrencyTrait, Nls, Usdc},
    price::{self, PriceDTO},
};
use marketprice::storage::Price;
use platform::coin_legacy::to_cosmwasm;

use crate::common::{test_case::TestCase, ADMIN};

type Currency = Usdc;
type TheCoin = Coin<Currency>;
const DENOM: &str = <Usdc as finance::currency::Currency>::SYMBOL;

fn create_coin(amount: u128) -> TheCoin {
    Coin::<Currency>::new(amount)
}

fn create_test_case() -> TestCase {
    let mut test_case = TestCase::with_reserve(DENOM, 10_000_000_000);
    test_case.init(
        &Addr::unchecked("user"),
        vec![to_cosmwasm(create_coin(1_000_000))],
    );
    test_case.init_lpp_with_funds(None, 5_000_000_000);
    test_case.init_timealarms();
    test_case.init_oracle(None);
    test_case.init_leaser();

    test_case
}

#[test]
fn internal_test_integration_setup_test() {
    let mut test_case = create_test_case();

    test_case
        .app
        .execute(
            Addr::unchecked(ADMIN),
            wasm_execute(
                test_case.oracle.clone().unwrap(),
                &oracle::msg::ExecuteMsg::RegisterFeeder {
                    feeder_address: ADMIN.into(),
                },
                vec![to_cosmwasm(create_coin(10000))],
            )
            .unwrap()
            .into(),
        )
        .unwrap();

    test_case
        .app
        .execute(
            Addr::unchecked(ADMIN),
            wasm_execute(
                test_case.oracle.clone().unwrap(),
                &oracle::msg::ExecuteMsg::FeedPrices {
                    prices: vec![PriceDTO::try_from(
                        price::total_of(Coin::<Usdc>::new(5)).is(Coin::<Nls>::new(7)),
                    )
                    .unwrap()],
                },
                vec![to_cosmwasm(create_coin(10000))],
            )
            .unwrap()
            .into(),
        )
        .expect("Oracle not properly connected!");
}
