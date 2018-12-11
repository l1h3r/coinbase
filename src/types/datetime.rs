use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;
use std::time::UNIX_EPOCH;

use crate::types::UtcDate;

#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTime(UtcDate);

impl Default for DateTime {
  fn default() -> Self {
    DateTime(UtcDate::from(UNIX_EPOCH))
  }
}

impl From<UtcDate> for DateTime {
  fn from(other: UtcDate) -> Self {
    DateTime(other)
  }
}

impl Deref for DateTime {
  type Target = UtcDate;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Debug for DateTime {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self.0)
  }
}

impl Display for DateTime {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)
  }
}
