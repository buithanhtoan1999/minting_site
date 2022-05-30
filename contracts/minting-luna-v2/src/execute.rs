
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, WhitelistInfo};
use crate::state::{Config, MintingContract, TokenIds};
use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, WasmMsg, Uint128,
};
use cw721::{Cw721ExecuteMsg};

impl<'a> MintingContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let owner = msg
            .owner
            .map_or(Ok(_info.sender), |o| deps.api.addr_validate(&o))?;

        let config = Config {
            owner: Some(owner),
            treasury: deps.api.addr_validate(&msg.treasury)?,
            nft_contract_address: deps.api.addr_validate(&msg.nft_contract_address)?,
            collection_name: msg.collection_name,
            collection_symbol: msg.collection_symbol,
            fee: msg.fee,
        };

        self.config.save(deps.storage, &config)?;

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
        
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::UpdateConfig { new_owner } => {
                self.execute_update_config(deps, _env, info, new_owner)
            }
            ExecuteMsg::AddWhiteList { whitelists } => self.execute_add_to_whitelist(deps, _env, info,whitelists),

            ExecuteMsg::Mint {} => self.execute_mint(deps, _env, info),
        }
    }
}

impl<'a> MintingContract<'a> {
    pub fn execute_update_config(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        new_owner: Option<String>,
    ) -> Result<Response, ContractError> {
        // authorize owner
        let cfg = self.config.load(deps.storage)?;
        let owner = cfg.owner.ok_or(ContractError::Unauthorized {})?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }

        // if owner some validated to addr, otherwise set to none
        let mut tmp_owner = None;
        if let Some(addr) = new_owner {
            tmp_owner = Some(deps.api.addr_validate(&addr)?)
        }

        self.config
            .update(deps.storage, |mut exists| -> StdResult<_> {
                exists.owner = tmp_owner;
                Ok(exists)
            })?;

        Ok(Response::new().add_attribute("action", "update_config"))
    }

    pub fn execute_mint(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let config = self.config.load(deps.storage)?;
        //check_whitelist

        let whitelist = self.token_whitelist.may_load(deps.storage, info.sender.as_bytes() )?;

        let mut messages: Vec<CosmosMsg> = vec![];

        if whitelist.is_none() {
            return Err(ContractError::WhitelistError {});

        }

        let token_whitelist_data = whitelist.unwrap();

        let length = Uint128::from(token_whitelist_data.token_ids.len() as u128);

        let cal_fee = config.fee * length;

        //check fee
        let sent = match info.funds.len() {
            0 => Err(StdError::generic_err(format!(
                "you need to send {}",
                &cal_fee
            ))),
            1 => {
                if info.funds[0].denom == "uluna" {
                    Ok(info.funds[0].amount)
                } else {
                    Err(StdError::generic_err(format!(
                        "you need to send {} to contract",
                        &cal_fee
                    )))
                }
            }
            _ => Err(StdError::generic_err(format!(
                "Only send {0} to register",
                &cal_fee
            ))),
        }?;

        if sent.is_zero() {
            return Err(ContractError::ZeroAmount {});
        }
        // Handle the player is not sending too much or too less
        if sent != cal_fee.clone() {
            return Err(ContractError::WrongAmount {});
        }

        for token_id in token_whitelist_data.token_ids.iter() {
            let transfer_nft_msg =    Cw721ExecuteMsg::TransferNft {
                recipient: info.sender.to_string(),
                token_id: token_id.clone(),
              };
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.nft_contract_address.to_string(),
                msg: to_binary(&transfer_nft_msg)?,
                funds: vec![]
              }));
        }

    

        Ok(
            Response::new().add_messages(messages)
                .add_attribute("action", "mint_nft")
                .add_attribute("sender", info.sender) 
        )
                             
    }

    pub fn execute_add_to_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        whitelists: Vec<WhitelistInfo>,

    ) -> Result<Response, ContractError> {
        // authorize owner
        let cfg = self.config.load(deps.storage)?;
        let owner = cfg.owner.ok_or(ContractError::Unauthorized {})?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }

        for whitelist in whitelists {

            self.token_whitelist.update(deps.storage, deps.api.addr_validate(&whitelist.owner)?.as_bytes(), |data| -> StdResult<TokenIds>{
                match data {
                    None => {
                        let token_id = TokenIds {
                            token_ids: vec![whitelist.token_id]
                        };
                        Ok(token_id)
                    }
                    Some(mut token_data) => {
                        token_data.token_ids.push(whitelist.token_id);
                        Ok(token_data)
                    },
                }
            })?;
        }

        Ok(Response::new().add_attribute("action", "add_whitelist"))
    }

}

