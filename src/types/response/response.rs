use crate::types::ErrorMessage;
use crate::types::Pagination;
use crate::types::WarningMessage;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Response<T: Default> {
  pub data: T,
  pub pagination: Option<Pagination>,
  pub errors: Vec<ErrorMessage>,
  pub warnings: Vec<WarningMessage>,
}
