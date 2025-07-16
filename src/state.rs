use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{ReadonlySingleton, Singleton, singleton, singleton_read};
use schemars::JsonSchema;
use secret_toolkit::storage::{Keymap, KeymapBuilder};
use serde::{Deserialize, Serialize};

/*TODO
secret-business-card-workshop
secret-dev-workshop

pub const EXECUTE_INCREMENT_REPLY_ID: u64 = 1;
let mut user_random = USER_RANDOM
      .may_load(deps.storage)?
      .unwrap_or(Random { random: random_u8 });
USER_RANDOM.save(deps.storage, &user_random)?;
*/

//---------== singleton
//pub const TOTAL: Item<u64> = Item::new(TOTAL_KEY);

pub static CONFIG_KEY: &[u8] = b"config";

//use u128 in storage
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
  pub count: u64,
  pub owner: Addr,
  pub flip: Vec<u8>,
}

//Singleton<State>: auto serializes or deserializes the State struct
pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
  singleton(storage, CONFIG_KEY)
}
pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
  singleton_read(storage, CONFIG_KEY)
}

//---------== KeyMap
pub static USERS: Keymap<Addr, User> = Keymap::new(b"users");

pub static ADDR_VOTE: Keymap<Addr, User> =
  KeymapBuilder::new(b"page_vote").with_page_size(13).build();

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
  pub name: String,
  pub password: String,
  pub balance: u128,
  pub updated_at: u64,
}
