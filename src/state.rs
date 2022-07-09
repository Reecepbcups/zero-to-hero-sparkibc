use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// we rename State -> Config bc it can get confusing with this being state.rs

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    // pub = public, Addr is a wallet address (like juno10r39fueph9fq7a6lgswu4zdsg8t3gxlq670lt0)
    pub admin_address: Addr,
}

// we can get the data like config.admint_address

// an Item of State which is in the storage_key "state".
// This key HAS to be unique like a KV Store
// CONFIG is all caps = Constant. So in storage it will be prefixed with "config" for the section
// This data is stored on the blockchain
pub const CONFIG: Item<Config> = Item::new("config");
