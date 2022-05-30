#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coin, from_binary, to_binary, CosmosMsg, DepsMut, Empty, Response, WasmMsg};

use crate::state::TokenIds;
use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, MintingContract, Stage},
};
const OWNER: &str = "merlin";
const TREASURY: &str = "treasury";
const NFT_CONTRACT_ADDRESS: &str = "astro_game";
const COLLECTION_NAME: &str = "astro_hero";
const COLLECTION_SYMBOL: &str = "ASTRO";

fn setup_contract(deps: DepsMut<'_>) -> MintingContract<'static> {
    let STAGES_0: Vec<Stage> = vec![
        Stage {
            stage_name: "Private Sale".to_string(),
            stage_id: 1,
            start_time: 1571797400,
            end_time: 1571797519,
            check_whitelisted: true,
            minting_price: coin(12, "ust"),
        },
        Stage {
            stage_name: "Public Sale".to_string(),
            stage_id: 1,
            start_time: 1653303,
            end_time: 1650324,
            check_whitelisted: false,
            minting_price: coin(23, "ust"),
        },
    ];

    let contract = MintingContract::default();
    let msg = InstantiateMsg {
        owner: Some(OWNER.to_string()),
        treasury: TREASURY.to_string(),
        nft_contract_address: NFT_CONTRACT_ADDRESS.to_string(),
        collection_name: COLLECTION_NAME.to_string(),
        collection_symbol: COLLECTION_SYMBOL.to_string(),
        minting_stages: STAGES_0,
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let stages_1: Vec<Stage> = vec![
        Stage {
            stage_name: "Private Sale".to_string(),
            stage_id: 1,
            start_time: 1630202,
            end_time: 1630429,
            check_whitelisted: true,
            minting_price: coin(12, "ust"),
        },
        Stage {
            stage_name: "Public Sale".to_string(),
            stage_id: 1,
            start_time: 1653303,
            end_time: 1650324,
            check_whitelisted: false,
            minting_price: coin(23, "ust"),
        },
    ];

    let mut deps = mock_dependencies(&[]);
    let contract = MintingContract::default();

    let msg = InstantiateMsg {
        owner: Some(OWNER.to_string()),
        treasury: TREASURY.to_string(),
        nft_contract_address: NFT_CONTRACT_ADDRESS.to_string(),
        collection_name: COLLECTION_NAME.to_string(),
        collection_symbol: COLLECTION_SYMBOL.to_string(),
        minting_stages: stages_1.clone(),
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
    assert_eq!(stages_1.clone(), config.minting_stages);
}

#[test]

fn add_to_whitelist() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());
    let whitelist_address = vec![
        "white_list1".to_string(),
        "white_list2".to_string(),
        "white_list3".to_string(),
    ];

    let stage_id: u8 = 1;
    let add_to_whitelist_msg = ExecuteMsg::AddToWhiteList {
        add_to_whitelists: whitelist_address,
        stage_id: stage_id,
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

    let res: bool = from_binary(
        &contract
            .query(
                deps.as_ref(),
                QueryMsg::Whitelist {
                    address: "white_list1".to_string(),
                    stage_id: 1,
                },
            )
            .unwrap(),
    )
    .unwrap();

    assert_eq!(res, true);
}

#[test]
fn add_token_ids() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());
    let token_ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];

    let add_to_token_ids_list_msg = ExecuteMsg::AddTokenIds {
        token_ids: token_ids,
    };

    //random can add whitelist

    let random = mock_info("random", &[]);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            random,
            add_to_token_ids_list_msg.clone(),
        )
        .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});

    //owner can add whitelist

    let owner = mock_info(OWNER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            owner.clone(),
            add_to_token_ids_list_msg.clone(),
        )
        .unwrap();

    let res = contract
        .query(deps.as_ref(), QueryMsg::TokenIdsList {})
        .unwrap();

    let token_ids_list: TokenIds = from_binary(&res).unwrap();
    assert_eq!(
        token_ids_list.token_ids,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
    let token_ids_2 = vec!["4".to_string(), "5".to_string(), "6".to_string()];
    let add_to_token_ids_list_msg_2 = ExecuteMsg::AddTokenIds {
        token_ids: token_ids_2,
    };

    let __ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            owner.clone(),
            add_to_token_ids_list_msg_2.clone(),
        )
        .unwrap();

    let res_2 = contract
        .query(deps.as_ref(), QueryMsg::TokenIdsList {})
        .unwrap();

    let token_ids_list_2: TokenIds = from_binary(&res_2).unwrap();

    assert_eq!(
        token_ids_list_2.token_ids,
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string()
        ]
    );
}

#[test]
fn random_mint() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());

    let token_ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];

    let add_to_token_ids_list_msg = ExecuteMsg::AddTokenIds {
        token_ids: token_ids,
    };

    let owner = mock_info(OWNER, &[]);
    //add token id
    contract
        .execute(
            deps.as_mut(),
            mock_env(),
            owner.clone(),
            add_to_token_ids_list_msg.clone(),
        )
        .unwrap();

    let random_mint_msg = ExecuteMsg::RandomMint {};

    let user_1 = mock_info("user", &[coin(12, "ust")]);

    let __ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            user_1.clone(),
            random_mint_msg.clone(),
        )
        .unwrap();

}
