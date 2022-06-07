use cosmwasm_std::{Uint128, Uint256};

use super::Integer;

impl Integer for Uint128 {
    type DoubleInteger = Uint256;
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Uint128;

    use crate::percent::test::{test_are, test_of, test_of_are};

    #[test]
    fn of_are() {
        test_of_are(1200, Uint128::from(50u32), Uint128::from(60u8));
        test_of_are(12, Uint128::from(500u16), Uint128::from(6u8));
        test_of_are(1000, Uint128::MAX, Uint128::MAX);
    }

    #[test]
    #[should_panic]
    fn of_overflow() {
        test_of(1001, Uint128::MAX, Uint128::MAX);
    }

    #[test]
    #[should_panic]
    fn are_overflow() {
        test_are(999, Uint128::MAX, Uint128::MAX);
    }
    #[test]
    #[should_panic]
    fn are_div_zero() {
        test_are(0, Uint128::MAX, Uint128::MAX);
    }
}