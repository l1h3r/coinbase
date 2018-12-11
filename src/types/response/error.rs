#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorMessage {
  pub id: String,
  pub message: String,
  pub url: Option<String>,
}
