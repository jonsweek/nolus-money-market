use cosmwasm_std::{to_binary, Addr, Api, Coin, Reply, StdResult, SubMsg, WasmMsg};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::msg::ExecuteMsg;

pub const REPLY_ID: u64 = 28;

pub trait Lpp: Serialize + DeserializeOwned {
    fn open_loan_req(&self, amount: Coin) -> StdResult<SubMsg>;
    fn open_loan_resp(&self, resp: Reply) -> Result<(), String>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LppStub {
    addr: Addr,
}

impl LppStub {
    pub fn try_from<A>(addr_raw: String, api: &A) -> StdResult<Self>
    where
        A: ?Sized + Api,
    {
        let addr = api.addr_validate(&addr_raw)?;
        Ok(Self { addr })
    }
}

impl Lpp for LppStub {
    fn open_loan_req(&self, amount: Coin) -> StdResult<SubMsg> {
        let msg = to_binary(&ExecuteMsg::OpenLoan {
            amount,
        })?;
        Ok(SubMsg::reply_on_success(
            WasmMsg::Execute {
                contract_addr: self.addr.as_ref().into(),
                funds: vec![],
                msg,
            },
            REPLY_ID,
        ))
    }

    fn open_loan_resp(&self, resp: Reply) -> Result<(), String> {
        debug_assert_eq!(REPLY_ID, resp.id);
        resp.result.into_result().map(|_| ())
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{from_binary, Addr, Coin, CosmosMsg, ReplyOn, WasmMsg};

    use crate::{msg::ExecuteMsg, stub::REPLY_ID};

    use super::{Lpp, LppStub};

    #[test]
    fn open_loan_req() {
        let addr = Addr::unchecked("defd2r2");
        let lpp = LppStub { addr: addr.clone() };
        let borrow_amount = Coin::new(10, "WRT");
        let msg = lpp
            .open_loan_req(borrow_amount.clone())
            .expect("open new loan request failed");
        assert_eq!(REPLY_ID, msg.id);
        assert_eq!(ReplyOn::Success, msg.reply_on);
        if let CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            funds,
            msg,
        }) = msg.msg
        {
            assert_eq!(addr, contract_addr);
            assert_eq!(Vec::<Coin>::new(), funds);
            let _lpp_msg: ExecuteMsg = from_binary(&msg).expect("invalid Lpp message");
            if let ExecuteMsg::OpenLoan{amount} = _lpp_msg {
                assert_eq!(borrow_amount, amount);
            } else {
                panic!("Bad Lpp message type!");    
            }
        } else {
            panic!("Bad Cosmos message!");
        }
    }
}