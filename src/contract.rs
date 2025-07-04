use schemars::JsonSchema;
use std::fmt;

use crate::{
  error::ContractError,
  msg::{ExecuteMsg, InstantiateMsg, PassowrdResponse, QueryMsg},
  state::{PASSWORD, Password, config},
};
use cosmwasm_std::{
  BankMsg, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, ensure,
  ensure_ne, entry_point, to_binary,
};

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
  password_key: String,
  password_value: String,
) -> StdResult<Response> /*Result<Response, ContractError>*/ {
  let password = Password {
    password: password_value.clone(),
  };
  //PASSWORD.insert(deps.storage, &password_key, &password)?;
  //deps.api.debug("password stored successfully");
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
