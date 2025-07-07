//use schemars::JsonSchema;
//use std::fmt;

use crate::{
  //error::ContractError,
  msg::{ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg, UserResp},
  state::{ADDR_VOTE, USERS, User, config},
};
use cosmwasm_std::{
  Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, entry_point,
  to_binary,
}; //ensure, ensure_ne, BankMsg, DepsMut

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
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response>
/*Result<Response<Empty>, ContractError>*/ {
  match msg {
    ExecuteMsg::StorePassword {
      password_key,
      password_value,
    } => try_store_password(deps, env, info, password_key, password_value),
    ExecuteMsg::Increment {} => try_increment(deps, env),
    ExecuteMsg::Reset { count } => try_reset(deps, env, count),
  }
}

pub fn try_store_password(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  password_key: String,
  password_value: String,
) -> StdResult<Response> /*Result<Response, ContractError>*/ {
  deps.api.debug("try_store_password");
  let sender: Addr = info.sender;
  deps.api.debug(sender.as_str());

  let password = User {
    password: password_value.clone(),
  };
  //ADDR_VOTE.insert(deps.storage, &sender, &password)?;
  USERS.insert(deps.storage, &password_key, &password)?;
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
fn query_password(deps: Deps, password_key: String) -> StdResult<UserResp> {
  let user = USERS
    .get(deps.storage, &password_key)
    .ok_or(StdError::generic_err("password_key incorrect"))?;

  let resp = UserResp {
    password: user.password,
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

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::{
    Api, Coin, StdResult, Uint128, from_binary,
    testing::{
      MockStorage, mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
    },
  }; //from_binary, Coin, Uint128, from_binary
  use secret_toolkit::storage::{Item, Keymap}; //https://github.com/scrtlabs/secret-toolkit/tree/master/packages/storage
  use serde::{Deserialize, Serialize};

  #[test]
  fn store_password() {
    let mut deps = mock_dependencies_with_balance(&[Coin {
      denom: "token".to_owned(),
      amount: Uint128::new(2),
    }]);
    let info = mock_info(
      "owner",
      &[Coin {
        denom: "token".to_owned(),
        amount: Uint128::new(2),
      }],
    );
    let init_msg = InstantiateMsg {}; //instantiate the contract
    let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

    //User1 stores password
    let info = mock_info(
      "user1",
      &[Coin {
        denom: "token".to_owned(),
        amount: Uint128::new(2),
      }],
    );
    let password1 = "pw1".to_owned();
    let msg = ExecuteMsg::StorePassword {
      password_key: "user1".to_owned(),
      password_value: password1.clone(),
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //read password
    let msg = QueryMsg::GetPassword {
      password_key: "user1".to_owned(),
    };
    let res: Binary = query(deps.as_ref(), mock_env(), msg).unwrap();
    let user: UserResp = from_binary(&res).unwrap();
    println!("Queried password: {}", user.password);
    assert_eq!(password1, user.password);
  }

  #[test]
  fn greet_query() {
    let name = "John".to_owned();
    let mut deps = mock_dependencies();

    let _res = instantiate(
      deps.as_mut(),
      mock_env(),
      mock_info("addr0", &[]),
      InstantiateMsg {}, //Empty {},
    )
    .unwrap();

    let resp = greet(deps.as_ref(), mock_env(), name.clone()).unwrap();
    //from_binary(&resp).unwrap();

    assert_eq!(
      resp,
      GreetResp {
        greet: format!("Hello {}", name)
      }
    );
  }
}
