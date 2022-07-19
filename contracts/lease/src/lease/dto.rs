use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::Item;
use finance::{currency::SymbolOwned, liability::Liability};
use serde::{Deserialize, Serialize};

use crate::loan::LoanDTO;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaseDTO {
    pub(super) customer: Addr,
    pub(super) currency: SymbolOwned,
    pub(super) liability: Liability,
    pub(super) loan: LoanDTO,
}

impl<'a> LeaseDTO {
    const DB_ITEM: Item<'a, LeaseDTO> = Item::new("lease");

    pub(crate) fn new(
        customer: Addr,
        currency: SymbolOwned,
        liability: Liability,
        loan: LoanDTO,
    ) -> Self {
        Self {
            customer,
            currency,
            liability,
            loan,
        }
    }

    pub(crate) fn store(&self, storage: &mut dyn Storage) -> StdResult<()> {
        Self::DB_ITEM.save(storage, self)
    }

    pub(crate) fn load(storage: &dyn Storage) -> StdResult<Self> {
        Self::DB_ITEM.load(storage)
    }
}