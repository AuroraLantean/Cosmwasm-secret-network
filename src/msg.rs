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
  StorePassword {
    password_key: String,
    password_value: String,
  },
  Flip {},
}

//-----== Query
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  Greet { name: String },
  GetCount {},
  GetPassword { password_key: String },
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
  pub password: String,
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
