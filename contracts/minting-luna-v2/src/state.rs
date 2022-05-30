use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map,};

pub struct MintingContract<'a> {
       pub config: Item<'a, Config>,
       pub token_whitelist: Map<'a,  &'a [u8], TokenIds>,
 }
     
impl Default for MintingContract<'static> {
    fn default() -> Self {
         Self::new(
           "config",
           "token_whitelist",
         )
       }
}

impl<'a> MintingContract<'a> {
       fn new(
         config_key: &'a str,
         token_whitelist_key: &'a str,
       ) -> Self {
         Self {
           config: Item::new(config_key),
           token_whitelist: Map::new(token_whitelist_key),
         }
       }
     }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
       /// Owner If None set, contract is frozen.
       pub owner: Option<Addr>,
       // Address who receipt luna for minting 
       pub treasury: Addr,
       //Address of contract nft
       pub nft_contract_address: Addr,
       //Collection Name
       pub collection_name: String,
       //collection symbol 
       pub collection_symbol: String,
       //Fee
        pub fee: Uint128

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenIds {
  pub token_ids: Vec<String>

}
