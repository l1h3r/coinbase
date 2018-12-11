use crate::deserialize_f64;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Currency {
  pub id: String,
  pub name: String,
  #[serde(deserialize_with = "deserialize_f64")]
  pub min_size: f64,
}
