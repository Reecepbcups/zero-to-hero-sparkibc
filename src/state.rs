use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

// an Item of State which is in the storage_key "state".
// This key HAS to be unique like a KV Store
pub const STATE: Item<State> = Item::new("state");
