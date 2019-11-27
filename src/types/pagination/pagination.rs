use crate::types::Order;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pagination {
  pub limit: usize,
  pub order: Order,
  pub ending_before: Option<String>,
  pub starting_after: Option<String>,
  pub previous_uri: Option<String>,
  pub next_uri: Option<String>,
  //
  // Undocumented
  //
  pub previous_ending_before: Option<String>,
  pub next_starting_after: Option<String>,
}

impl Default for Pagination {
  fn default() -> Self {
    Self {
      limit: 25,
      order: Order::default(),
      ending_before: None,
      starting_after: None,
      previous_uri: None,
      next_uri: None,
      previous_ending_before: None,
      next_starting_after: None,
    }
  }
}

impl Pagination {
  pub fn to_query(&self) -> Vec<(&'static str, String)> {
    unimplemented!()
  }
}
