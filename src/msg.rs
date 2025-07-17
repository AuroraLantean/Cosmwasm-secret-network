use cosmwasm_std::{Addr, Uint128};
//inputs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//-----== Instantiate
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub count: u64,
  pub flip: Vec<u8>,
  //pub admin: String,
}

//-----== Execute by function signature
//prefer to use Uint128 in entrypoint messages
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  Increment {
    amt: u64,
  },
  Decrement {
    amt: u64,
  },
  Reset {
    count: u64,
  },
  AddUser {
    name: String,
    password: String,
  },
  Deposit {},
  RemoveUser {
    addr: Addr,
  },
  Flip {},
  Withdraw {
    denom: String,
    dest: Addr,
    amount: Uint128,
  },
  CrossContract {
    contract_addr: String,
    code_hash: String,
  },
}

//-----== Query
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  Greet { name: String },
  GetCount {},
  GetUser { addr: Addr },
  GetFlip {},
}

//-----== custom struct for each query response
//must have pub in fields!
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CountResp {
  pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserResp {
  pub name: String,
  pub password: String,
  pub balance: u128,
  pub updated_at: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GreetResp {
  pub greet: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct FlipResponse {
  pub flip: Vec<u8>,
}
