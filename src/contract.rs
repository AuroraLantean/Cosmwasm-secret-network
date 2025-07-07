//use schemars::JsonSchema;
//use std::fmt;

use crate::{
  //error::ContractError,
  msg::{ExecuteMsg, GreetResp, InstantiateMsg, PassowrdResp, QueryMsg},
  state::{PASSWORD, Password, config},
};
use cosmwasm_std::{
  Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, entry_point,
  to_binary,
}; //ensure, ensure_ne, BankMsg,

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  _deps: DepsMut,     //accessing contract states
  _env: Env,          //current blockchain state
  _info: MessageInfo, //sender addr and incoming native token
  _msg: InstantiateMsg,
) -> StdResult<Response> {
  Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
//#[entry_point]
pub fn execute(
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
  msg: ExecuteMsg,
) -> StdResult<Response>
/*Result<Response<Empty>, ContractError>*/ {
  match msg {
    ExecuteMsg::StorePassword {
      password_key,
      password_value,
    } => try_store_password(deps, env, password_key, password_value),
    ExecuteMsg::Increment {} => try_increment(deps, env),
    ExecuteMsg::Reset { count } => try_reset(deps, env, count),
  }
}

pub fn try_store_password(
  deps: DepsMut,
  _env: Env,
  _password_key: String,
  password_value: String,
) -> StdResult<Response> /*Result<Response, ContractError>*/ {
  let _password = Password {
    password: password_value.clone(),
  };
  //PASSWORD.insert(deps.storage, &password_key, &password)?; //expected mutable reference `&mut dyn cosmwasm_std::traits::Storage`   found mutable reference `&mut dyn Storage
  deps.api.debug("password stored successfully");
  Ok(Response::default())
}

pub fn try_increment(deps: DepsMut, _env: Env) -> StdResult<Response> {
  config(deps.storage).update(|mut state| -> Result<_, StdError> {
    state.count += 1;
    Ok(state)
  })?;
  deps.api.debug("count incremented successfully");
  Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, _env: Env, count: i32) -> StdResult<Response> {
  config(deps.storage).update(|mut state| -> Result<_, StdError> {
    state.count = count;
    Ok(state)
  })?;
  deps.api.debug("count incremented successfully");
  Ok(Response::default())
}

//query always returns serialized data
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
  use QueryMsg::*;
  match msg {
    Greet { name } => to_binary(&greet(deps, env, name)?),
    GetCount {} => to_binary(&query_count(deps)?),
    GetPassword { password_key } => to_binary(&query_password(deps, password_key)?),
  }
} //_msg: Empty ... an empty JSON
fn greet(_deps: Deps, _env: Env, name: String) -> StdResult<GreetResp> {
  let resp = GreetResp {
    greet: format!("Hello {}", name), //"Hello World".to_owned(),
  };
  Ok(resp)
}
fn query_password(_deps: Deps, _password_key: String) -> StdResult<PassowrdResp> {
  let resp = PassowrdResp {
    password: "fake_password".to_owned(),
  };
  Ok(resp)
}
fn query_count(_deps: Deps) -> StdResult<i32> {
  let count = 0;
  Ok(count)
}
/*fn query_count(deps: Deps) -> StdResult<TotalWeightResponse> {
  let weight = TOTAL.load(deps.storage)?;
  Ok(TotalWeightResponse { weight })
}*/

// cargo test
#[cfg(test)]
mod tests {
  use super::*;
  //use cosmwasm_std::from_binary;
  use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

  #[test]
  fn greet_query() {
    let name = "John".to_owned();
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
      deps.as_mut(),
      env.clone(),
      mock_info("addr0", &[]),
      InstantiateMsg {}, //Empty {},
    )
    .unwrap();

    let resp = greet(deps.as_ref(), env, name.clone()).unwrap();
    //from_binary(&resp).unwrap();
    println!("resp {:?}", resp);
    assert_eq!(
      resp,
      GreetResp {
        greet: format!("Hello {}", name)
      }
    );
  }
}
