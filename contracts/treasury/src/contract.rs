#[cfg(feature = "cosmwasm-bindings")]
use cosmwasm_std::entry_point;

use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, Storage};
use cw2::set_contract_version;
use platform::bank::{BankAccount, BankStub};
use finance::coin::Coin;
use finance::currency::Nls;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{self, ADMIN, REWARDS_DISPATCHER};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = info.sender;
    ADMIN.save(deps.storage, &admin)?;

    Ok(Response::default())
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let sender = info.sender;
    match msg {
        ExecuteMsg::ConfigureRewardTransfer { rewards_dispatcher } => {
            try_configure_reward_transfer(deps, sender, rewards_dispatcher)
        }
        ExecuteMsg::SendRewards { amount } => {
            let bank_account = BankStub::my_account(&env, &deps.querier);

            try_send_rewards(deps.storage, sender, amount, bank_account)
        }
    }
}

fn try_configure_reward_transfer(
    deps: DepsMut,
    sender: Addr,
    rewards_dispatcher: Addr,
) -> Result<Response, ContractError> {
    state::assert_admin(deps.storage, sender)?;
    deps.api.addr_validate(rewards_dispatcher.as_str())?;
    REWARDS_DISPATCHER.save(deps.storage, &rewards_dispatcher)?;
    Ok(Response::new().add_attribute("method", "try_configure_reward_transfer"))
}

fn try_send_rewards<B>(
    storage: &mut dyn Storage,
    sender: Addr,
    amount: Coin<Nls>,
    account: B,
) -> Result<Response, ContractError>
where
    B: BankAccount,
{
    state::assert_rewards_dispatcher(storage, &sender)?;
    let pay_msg = account.send(amount, &sender)?;

    let response = Response::new()
        .add_attribute("method", "try_send_rewards")
        .add_submessage(pay_msg);

    Ok(response)
}
