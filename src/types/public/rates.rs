use crate::types::SimpleMap;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rates {
  pub currency: String,
  // TODO: f64 values
  pub rates: SimpleMap,
}
