//use schemars::JsonSchema;
//use std::fmt;
use crate::error::ContractError;
use crate::{
  //error::ContractError,
  msg::{CountResp, ExecuteMsg, FlipResponse, GreetResp, InstantiateMsg, QueryMsg, UserResp},
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
    flip: msg.flip,
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
    ExecuteMsg::AddUser {
      name,
      password,
      balance,
    } => try_add_user(deps, env, info, name, password, balance),
    ExecuteMsg::Deposit { amount } => try_deposit(deps, env, info, amount),
    ExecuteMsg::Increment { amt } => try_increment(deps, env, amt),
    ExecuteMsg::Decrement { amt } => try_decrement(deps, env, amt),
    ExecuteMsg::Reset { count } => try_reset(deps, env, info, count),
    ExecuteMsg::Flip {} => try_flip(deps, env),
  }
}
pub fn try_flip(deps: DepsMut, env: Env) -> StdResult<Response> {
  config(deps.storage).update(|mut state| -> Result<_, StdError> {
    let coin_flip: Vec<u8> = env.block.random.unwrap().0;
    state.flip = coin_flip;
    Ok(state)
  })?;
  deps.api.debug("flipped!");
  Ok(Response::default())
}

pub fn try_deposit(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  amount: u64,
) -> StdResult<Response> /*Result<Response, ContractError>*/
{
  deps.api.debug("try_deposit");
  let sender = info.sender.clone();

  let mut user = USERS
    .get(deps.storage, &sender)
    .ok_or(StdError::generic_err("name incorrect"))?;

  user.balance += amount;
  USERS.insert(deps.storage, &sender, &user)?;
  deps.api.debug("password stored successfully");
  Ok(Response::default())
}

pub fn try_add_user(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  name: String,
  password: String,
  balance: u64,
) -> StdResult<Response> /*Result<Response, ContractError>*/ {
  deps.api.debug("try_add_user");
  let sender: Addr = info.sender;
  deps.api.debug(sender.as_str());

  let user = User {
    name,
    password: password.clone(),
    balance,
    updated_at: env.block.time.seconds(),
  };
  USERS.insert(deps.storage, &sender, &user)?;
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
    GetUser { addr } => to_binary(&query_user(deps, addr)?),
    GetFlip {} => to_binary(&query_flip(deps)?),
  }
} //_msg: Empty ... an empty JSON
fn query_flip(deps: Deps) -> StdResult<FlipResponse> {
  let state = config_read(deps.storage).load()?;
  Ok(FlipResponse { flip: state.flip })
}
fn greet(deps: Deps, env: Env, name: String) -> StdResult<GreetResp> {
  let str = format!("greet() at time:{}", env.block.time.seconds());
  deps.api.debug(&str);
  let resp = GreetResp {
    greet: format!("Hello {}", name), //"Hello World".to_owned(),
  };
  Ok(resp)
}
fn query_user(deps: Deps, addr: Addr) -> StdResult<UserResp> {
  let user = USERS
    .get(deps.storage, &addr)
    .ok_or(StdError::generic_err("name incorrect"))?;

  let resp = UserResp {
    name: user.name,
    password: user.password,
    balance: user.balance,
    updated_at: user.updated_at,
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
  use cosmwasm_std::{Coin, StdError, StdResult, Uint128, from_binary};
  use cosmwasm_std::{Timestamp, testing::*};
  use secret_toolkit::storage::{Item, Keymap}; //https://github.com/scrtlabs/secret-toolkit/tree/master/packages/storage

  fn set_time(sec_since_epoc: u64) -> Env {
    let mut env = mock_env();
    env.block.time = Timestamp::from_nanos(sec_since_epoc * 1000000000);
    env
  }
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
    let init_msg = InstantiateMsg {
      count: 17,
      flip: vec![1, 2, 3],
    };

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
    let init_msg = InstantiateMsg {
      count: 17,
      flip: vec![1, 2, 3],
    };

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
    let init_msg = InstantiateMsg {
      count: 17,
      flip: vec![1, 2, 3],
    };

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
  fn add_user() {
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
    let init_msg = InstantiateMsg {
      count: 0,
      flip: vec![1, 2, 3],
    }; //instantiate the contract

    let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

    //User1 stores password
    let user1 = Addr::unchecked("user1");
    let info = mock_info(
      "user1",
      &[Coin {
        denom: "token".to_owned(),
        amount: Uint128::new(2),
      }],
    );
    let password1 = "pw1".to_owned();
    let balance = 122;
    let msg = ExecuteMsg::AddUser {
      name: "user1".to_owned(),
      password: password1.clone(),
      balance,
    };
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    //read user
    let msg = QueryMsg::GetUser {
      addr: user1.clone(),
    };
    let res: Binary = query(deps.as_ref(), mock_env(), msg).unwrap();
    let user: UserResp = from_binary(&res).unwrap();
    println!("Queried user: {:?}", user);
    assert_eq!("user1".to_owned(), user.name);
    assert_eq!(password1, user.password);
    assert_eq!(balance, user.balance);

    //--------== update balance
    let amount = 15;
    let msg = ExecuteMsg::Deposit { amount };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = QueryMsg::GetUser { addr: user1 };
    let res: Binary = query(deps.as_ref(), mock_env(), msg).unwrap();
    let user: UserResp = from_binary(&res).unwrap();
    println!("Queried user: {:?}", user);
    assert_eq!(balance + amount, user.balance);
  }

  #[test]
  fn greet_query() {
    let name = "John".to_owned();
    let mut deps = mock_dependencies();

    let _res = instantiate(
      deps.as_mut(),
      mock_env(),
      mock_info("addr0", &[]),
      InstantiateMsg {
        count: 0,
        flip: vec![1, 2, 3],
      }, //Empty {},
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

    let env = set_time(1752489600);
    let resp = greet(deps.as_ref(), env, name.clone()).unwrap();
  }
}
