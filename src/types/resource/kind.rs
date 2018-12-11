#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
  Account,
  Address,
  Buy,
  Deposit,
  Notification,
  PaymentMethod,
  Sell,
  Transaction,
  User,
  Withdrawal,
  #[serde(other)]
  Unknown,
}

impl Default for ResourceType {
  fn default() -> Self {
    ResourceType::Unknown
  }
}
