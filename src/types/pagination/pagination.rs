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
    let mut query: Vec<(&'static str, String)> = vec![
      ("limit", self.limit.to_string()),
      ("order", self.order.to_string()),
    ];

    if let Some(ref value) = self.ending_before {
      query.push(("ending_before", value.to_owned()));
    // } else if let Some(ref value) = self.previous_ending_before {
    //   query.push(("ending_before", value.to_owned()));
    }

    if let Some(ref value) = self.starting_after {
      query.push(("starting_after", value.to_owned()));
    // } else if let Some(ref value) = self.next_starting_after {
    //   query.push(("starting_after", value.to_owned()));
    }

    query
  }
}
