use crate::types::DateTime;
use crate::types::ResourceType;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceMeta {
  pub id: String,
  pub resource: ResourceType,
  pub resource_path: String,
  pub created_at: Option<DateTime>,
  pub updated_at: Option<DateTime>,
}
