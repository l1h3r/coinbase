use crate::types::DateTime;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Time {
  pub iso: DateTime,
  pub epoch: f64,
}
