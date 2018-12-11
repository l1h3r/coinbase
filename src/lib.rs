#![allow(clippy::module_inception)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;
mod utils;

pub mod client;
pub mod error;
pub mod types;

use self::utils::*;

pub mod prelude {
  pub use crate::client::Client;
  pub use crate::error::Error;
  pub use crate::types::*;
}
