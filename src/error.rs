use crypto_mac::InvalidKeyLength;
use reqwest::header::InvalidHeaderValue;
use reqwest::Error as Reqwest;
use serde::Deserialize;
use serde::Deserializer;
use serde_json::Error as Serde;

#[derive(Debug)]
pub enum Error {
  HTTP(Reqwest),
  JSON { error: Serde, data: Option<String> },
  InvalidAuth,
}

impl From<InvalidKeyLength> for Error {
  fn from(_: InvalidKeyLength) -> Self {
    Error::InvalidAuth
  }
}

impl From<InvalidHeaderValue> for Error {
  fn from(_: InvalidHeaderValue) -> Self {
    Error::InvalidAuth
  }
}

impl From<Serde> for Error {
  fn from(other: Serde) -> Self {
    Error::JSON {
      error: other,
      data: None,
    }
  }
}

impl From<Reqwest> for Error {
  fn from(other: Reqwest) -> Self {
    Error::HTTP(other)
  }
}

impl<'de> Deserialize<'de> for Error {
  fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
    unimplemented!();
  }
}
