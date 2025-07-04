use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{ReadonlySingleton, Singleton, singleton, singleton_read};
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

//---------== singleton
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
pub static PASSWORD: Keymap<String, Password> = Keymap::new(b"password");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Password {
  pub password: String,
}
