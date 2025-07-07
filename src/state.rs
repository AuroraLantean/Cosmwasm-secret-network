use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{ReadonlySingleton, Singleton, singleton, singleton_read};
use schemars::JsonSchema;
use secret_toolkit::storage::{Keymap, KeymapBuilder};
use serde::{Deserialize, Serialize};

//---------== singleton
//pub const TOTAL: Item<u64> = Item::new(TOTAL_KEY);

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
  pub count: i32,
  pub owner: Addr,
}

//Singleton<State>: auto serializes or deserializes the State struct
pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
  singleton(storage, CONFIG_KEY)
}

//---------== KeyMap
pub static USERS: Keymap<String, User> = Keymap::new(b"password");

pub static ADDR_VOTE: Keymap<Addr, User> =
  KeymapBuilder::new(b"page_vote").with_page_size(13).build();

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
  pub password: String,
}
