//use schemars::JsonSchema;
//use std::fmt;
use crate::{
  //error::ContractError,
  msg::{CountResp, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg, UserResp},
  state::{ADDR_VOTE, State, USERS, User, config, config_read},
};
use cosmwasm_std::{
  Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, entry_point,
  to_binary,
}; //ensure, ensure_ne, BankMsg, DepsMut

//#[cfg_attr(not(feature = "library"), entry_point)]
#[entry_point]
pub fn instantiate(
  deps: DepsMut,     //accessing contract states
  _env: Env,         //current blockchain state
  info: MessageInfo, //sender addr and incoming native token
  msg: InstantiateMsg,
) -> StdResult<Response> {
  let state = State {
    count: msg.count,
    owner: info.sender.clone(),
  };
  deps
    .api
    .debug(format!("Contract was initialized by {}", info.sender).as_str());
  config(deps.storage).save(&state)?;
  Ok(Response::default())
}

//#[cfg_attr(not(feature = "library"), entry_point)]
#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response>
/*Result<Response<Empty>, ContractError>*/ {
  match msg {
    ExecuteMsg::StorePassword {
      password_key,
      password_value,
    } => try_store_password(deps, env, info, password_key, password_value),
    ExecuteMsg::Increment { amt } => try_increment(deps, env, amt),
    ExecuteMsg::Decrement { amt } => try_decrement(deps, env, amt),
    ExecuteMsg::Reset { count } => try_reset(deps, env, info, count),
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

pub fn try_increment(deps: DepsMut, _env: Env, amt: u64) -> StdResult<Response> {
  config(deps.storage).update(|mut state| -> Result<_, StdError> {
    state.count += amt;
    Ok(state)
  })?;
  deps.api.debug("count incremented successfully");
  Ok(Response::default())
}
pub fn try_decrement(deps: DepsMut, _env: Env, amt: u64) -> StdResult<Response> {
  config(deps.storage).update(|mut state| -> Result<_, StdError> {
    if state.count < amt {
      return Err(StdError::generic_err(" count < amount"));
    };
    state.count -= amt;
    Ok(state)
  })?;
  deps.api.debug("count decremented successfully");
  Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, _env: Env, info: MessageInfo, count: u64) -> StdResult<Response> {
  let sender_address = info.sender.clone();

  config(deps.storage).update(|mut state| -> Result<_, StdError> {
    if sender_address != state.owner {
      return Err(StdError::generic_err("Only the owner can reset count"));
    };
    state.count = count;
    Ok(state)
  })?;
  deps.api.debug("count reset successfully");
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
fn query_count(deps: Deps) -> StdResult<CountResp> {
  let state = config_read(deps.storage).load()?;
  Ok(CountResp { count: state.count })
}
/*fn query_count(deps: Deps) -> StdResult<TotalWeightResponse> {
  let weight = TOTAL.load(deps.storage)?;
  Ok(TotalWeightResponse { weight })
}*/

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::testing::*;
  use cosmwasm_std::{Api, Coin, StdError, StdResult, Uint128, from_binary};
  use secret_toolkit::storage::{Item, Keymap}; //https://github.com/scrtlabs/secret-toolkit/tree/master/packages/storage

  #[test]
  fn test_query_count() {
    let mut deps = mock_dependencies();
    let info = mock_info(
      "owner0",
      &[Coin {
        denom: "token1".to_string(),
        amount: Uint128::new(1000),
      }],
    );
    let init_msg = InstantiateMsg { count: 17 };

    // we can just call .unwrap() to assert this was a success
    let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: CountResp = from_binary(&res).unwrap();
    assert_eq!(17, value.count);
  }

  #[test]
  fn test_increment_decrement() {
    let mut deps = mock_dependencies_with_balance(&[Coin {
      denom: "token".to_string(),
      amount: Uint128::new(2),
    }]);
    let info = mock_info(
      "owner1",
      &[Coin {
        denom: "token".to_string(),
        amount: Uint128::new(1000),
      }],
    );
    let init_msg = InstantiateMsg { count: 17 };

    let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

    // anyone can increment
    let info = mock_info(
      "anyone",
      &[Coin {
        denom: "token".to_string(),
        amount: Uint128::new(2),
      }],
    );
    //--------== Increment
    let exec_msg = ExecuteMsg::Increment { amt: 15 };
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), exec_msg).unwrap();

    // check count value
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: CountResp = from_binary(&res).unwrap();
    assert_eq!(32, value.count);

    //--------== Decrement
    let exec_msg = ExecuteMsg::Decrement { amt: 9 };
    let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

    // check count value
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: CountResp = from_binary(&res).unwrap();
    assert_eq!(23, value.count);
  }

  #[test]
  fn test_reset() {
    let mut deps = mock_dependencies_with_balance(&[Coin {
      denom: "token".to_string(),
      amount: Uint128::new(2),
    }]);
    let info = mock_info(
      "owner2",
      &[Coin {
        denom: "token".to_string(),
        amount: Uint128::new(2),
      }],
    );
    let init_msg = InstantiateMsg { count: 17 };

    let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

    // not anyone can reset
    let info = mock_info(
      "anyone",
      &[Coin {
        denom: "token".to_string(),
        amount: Uint128::new(2),
      }],
    );
    let exec_msg = ExecuteMsg::Reset { count: 5 };

    let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

    match res {
      Err(StdError::GenericErr { .. }) => {}
      _ => panic!("Must return unauthorized error"),
    }

    // only the owner2 can reset the counter
    let info = mock_info(
      "owner2",
      &[Coin {
        denom: "token".to_string(),
        amount: Uint128::new(2),
      }],
    );
    let exec_msg = ExecuteMsg::Reset { count: 5 };
    let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

    // should now be 5
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: CountResp = from_binary(&res).unwrap();
    assert_eq!(5, value.count);
  }

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
        amount: Uint128::new(1000),
      }],
    );
    let init_msg = InstantiateMsg { count: 0 }; //instantiate the contract

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
      InstantiateMsg { count: 0 }, //Empty {},
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
