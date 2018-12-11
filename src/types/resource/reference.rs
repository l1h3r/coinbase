use crate::types::ResourceType;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceRef {
  pub id: String,
  pub resource: ResourceType,
  pub resource_path: String,
}
