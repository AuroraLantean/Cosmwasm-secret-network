use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
  // let thiserror implement From<StdError>
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("Unauthorized")]
  Unauthorized {}, // when message sender != owner

  #[error("Custom Error val: {val:?}")]
  CustomError { val: String },
  // Add any other custom errors you like here.
  // https://docs.rs/thiserror/latest/thiserror/index.html
}
