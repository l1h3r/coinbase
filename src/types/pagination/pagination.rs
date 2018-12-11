use crate::types::Order;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pagination {
  pub limit: usize,
  pub order: Order,
  pub ending_before: Option<String>,
  pub starting_after: Option<String>,
  pub previous_uri: Option<String>,
  pub next_uri: Option<String>,
}
