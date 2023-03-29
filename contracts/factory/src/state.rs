use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub child_codeid: u64,
    pub children: Vec<String>,
}

pub const STATE: Item<State> = Item::new("state");