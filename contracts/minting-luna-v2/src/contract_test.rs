#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coin, from_binary, to_binary, CosmosMsg, DepsMut, Empty, Response, WasmMsg, Uint128};

use crate::msg::WhitelistInfo;
use crate::state::TokenIds;
use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, MintingContract},
};
const OWNER: &str = "merlin";
const TREASURY: &str = "treasury";
const NFT_CONTRACT_ADDRESS: &str = "astro_game";
const COLLECTION_NAME: &str = "astro_hero";
const COLLECTION_SYMBOL: &str = "ASTRO";

fn setup_contract(deps: DepsMut<'_>) -> MintingContract<'static> {


    let contract = MintingContract::default();
    let msg = InstantiateMsg {
        owner: Some(OWNER.to_string()),
        treasury: TREASURY.to_string(),
        nft_contract_address: NFT_CONTRACT_ADDRESS.to_string(),
        collection_name: COLLECTION_NAME.to_string(),
        collection_symbol: COLLECTION_SYMBOL.to_string(),
        fee: Uint128::from(30000u128),
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {

    let mut deps = mock_dependencies(&[]);
    let contract = MintingContract::default();

    let msg = InstantiateMsg {
        owner: Some(OWNER.to_string()),
        treasury: TREASURY.to_string(),
        nft_contract_address: NFT_CONTRACT_ADDRESS.to_string(),
        collection_name: COLLECTION_NAME.to_string(),
        collection_symbol: COLLECTION_SYMBOL.to_string(),
        fee: Uint128::from(30000u128),
    };
    let info = mock_info("creator", &[]);

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = contract.query(deps.as_ref(), QueryMsg::Config {}).unwrap();

    let config: Config = from_binary(&res).unwrap();
    assert_eq!("merlin", config.owner.unwrap().as_str());
    assert_eq!("treasury", config.treasury.as_str());
    assert_eq!("astro_game", config.nft_contract_address.as_str());
    assert_eq!("astro_hero", config.collection_name.as_str());
    assert_eq!("ASTRO", config.collection_symbol.as_str());
    assert_eq!( Uint128::from(30000u128), config.fee);
}

#[test]

fn add_to_whitelist() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());
    let whitelist_info = vec![
      WhitelistInfo {
          owner: "owner_1".to_string(),
          token_id: "token_id_1".to_string()
      },
      WhitelistInfo {
        owner: "owner_2".to_string(),
        token_id: "token_id_2".to_string()
    },
    WhitelistInfo {
        owner: "owner_3".to_string(),
        token_id: "token_id_3".to_string()
    },
    WhitelistInfo {
        owner: "owner_4".to_string(),
        token_id: "token_id_4".to_string()
    },
    WhitelistInfo {
        owner: "owner_5".to_string(),
        token_id: "token_id_5".to_string()
    },
    ];

    let add_to_whitelist_msg = ExecuteMsg::AddWhiteList {
        whitelists: whitelist_info,
    };

    //random can add whitelist

    let random = mock_info("random", &[]);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            random,
            add_to_whitelist_msg.clone(),
        )
        .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});

    //owner can add whitelist

    let owner = mock_info(OWNER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            owner,
            add_to_whitelist_msg.clone(),
        )
        .unwrap();

    let res: TokenIds = from_binary(
        &contract
            .query(
                deps.as_ref(),
                QueryMsg::Whitelists {
                    address: "owner_1".to_string(),

                },
            )
            .unwrap(),
    )
    .unwrap();
    let token_ids = TokenIds {
        token_ids : vec!["token_id_1".to_string()]
    };
    assert_eq!(res, token_ids);
}

#[test]
fn random_mint() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());

    let whitelist_info = vec![
        WhitelistInfo {
            owner: "owner_1".to_string(),
            token_id: "token_id_1".to_string()
        },
        WhitelistInfo {
          owner: "owner_2".to_string(),
          token_id: "token_id_2".to_string()
      },
      WhitelistInfo {
          owner: "owner_3".to_string(),
          token_id: "token_id_3".to_string()
      },
      WhitelistInfo {
          owner: "owner_4".to_string(),
          token_id: "token_id_4".to_string()
      },
      WhitelistInfo {
          owner: "owner_5".to_string(),
          token_id: "token_id_5".to_string()
      },
      ];
  
      let add_to_whitelist_msg = ExecuteMsg::AddWhiteList {
          whitelists: whitelist_info,
      };
      let owner = mock_info(OWNER, &[]);
      let _ = contract
          .execute(
              deps.as_mut(),
              mock_env(),
              owner,
              add_to_whitelist_msg.clone(),
          )
          .unwrap();
     
     let mint_msg = ExecuteMsg::Mint{};

     let sender = mock_info("owner_1", &[coin(30000, "uluna")]);
     let _res = contract
     .execute(
         deps.as_mut(),
         mock_env(),
         sender,
         mint_msg.clone(),
     )
     .unwrap();
     println!("res:  {:?}", _res);
}
