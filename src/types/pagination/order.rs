#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum Order {
  ASC,
  DESC,
}

impl Default for Order {
  fn default() -> Self {
    Order::DESC
  }
}
