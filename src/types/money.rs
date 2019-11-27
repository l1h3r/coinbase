use crate::deserialize_f64;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Money {
  #[serde(deserialize_with = "deserialize_f64")]
  pub amount: f64,
  pub currency: String,
  pub base: Option<String>,
  //
  // Undocumented
  //
  pub scale: Option<usize>,
}
