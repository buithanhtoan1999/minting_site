use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult, Order::Ascending as Ascending, };

use crate::msg::QueryMsg;
use crate::state::MintingContract;

impl<'a> MintingContract<'a> {
    pub fn query(&self, deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
      match msg {
        QueryMsg::Config {} => to_binary(&self.config.load(deps.storage)?),
        QueryMsg::Whitelists{address} => to_binary(&self.token_whitelist.load(deps.storage,  deps.api.addr_validate(&address)?.as_bytes())?),
      }
    }
  }




