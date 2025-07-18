use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
  // let thiserror implement From<StdError>
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("Unauthorized")]
  Unauthorized {}, // when message sender != owner

  #[error("Unknown id: {id:?}")]
  UnexpectedId { id: u64 },

  /// Whenever UTF-8 bytes cannot be decoded into a unicode string, e.g. in String::from_utf8 or str::from_utf8.
  #[error("Cannot decode UTF8 bytes into string: {msg}")]
  InvalidUtf8 { msg: String },
  // Add any other custom errors you like here.
  // https://docs.rs/thiserror/latest/thiserror/index.html
}
impl ContractError {
  pub fn invalid_utf8(msg: impl ToString) -> Self {
    ContractError::InvalidUtf8 {
      msg: msg.to_string(),
    }
  }
}
impl From<std::str::Utf8Error> for ContractError {
  fn from(source: std::str::Utf8Error) -> Self {
    Self::invalid_utf8(source)
  }
}
impl From<std::string::FromUtf8Error> for ContractError {
  fn from(source: std::string::FromUtf8Error) -> Self {
    Self::invalid_utf8(source)
  }
}
