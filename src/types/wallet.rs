use crate::types::DateTime;
use crate::types::Money;
use crate::types::ResourceMeta;
use crate::types::ResourceRef;
use crate::types::SimpleMap;
use crate::types::ValueMap;

///
/// TODO
///
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
  Wallet,
  Fiat,
  Vault,
  #[serde(other)]
  Unknown,
}

impl Default for AccountType {
  fn default() -> Self {
    AccountType::Unknown
  }
}

///
/// TODO
///
#[derive(Clone, Debug, Deserialize)]
pub enum PaymentMethodType {
  #[serde(rename = "ach_bank_account")]
  ACHBankAccount,
  #[serde(rename = "sepa_bank_account")]
  SEPABankAccount,
  #[serde(rename = "ideal_bank_account")]
  IDEALBankAccount,
  #[serde(rename = "fiat_account")]
  FiatAccount,
  #[serde(rename = "bank_wire")]
  BankWire,
  #[serde(rename = "credit_card")]
  CreditCard,
  #[serde(rename = "secure3d_card")]
  Secure3dCard,
  #[serde(rename = "eft_bank_account")]
  EFTBankAccount,
  #[serde(rename = "interac")]
  Interac,
  #[serde(other)]
  Unknown,
}

impl Default for PaymentMethodType {
  fn default() -> Self {
    PaymentMethodType::Unknown
  }
}

///
/// TODO
///
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransferStatus {
  Created,
  Completed,
  Canceled,
  #[serde(other)]
  Unknown,
}

impl Default for TransferStatus {
  fn default() -> Self {
    TransferStatus::Unknown
  }
}

///
/// TODO
///
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
  Pending,
  Completed,
  Failed,
  Expired,
  Canceled,
  WaitingForSignature,
  WaitingForClearing,
  #[serde(other)]
  Unknown,
}

impl Default for TransactionStatus {
  fn default() -> Self {
    TransactionStatus::Unknown
  }
}

///
/// TODO
///
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
  Send,
  Request,
  Transfer,
  Buy,
  Sell,
  FiatDeposit,
  FiatWithdrawal,
  ExchangeDeposit,
  ExchangeWithdrawal,
  VaultWithdrawal,
  #[serde(other)]
  Unknown,
}

impl Default for TransactionType {
  fn default() -> Self {
    TransactionType::Unknown
  }
}

///
/// TODO
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountCurrency {
  #[serde(rename = "type")]
  pub kind: String, // TODO: Enum
  pub name: String,
  pub address_regex: String,
  pub asset_id: String,
  pub code: String, // TODO: Enum?
  pub color: String,
  pub exponent: usize,
  pub sort_index: usize,
}

///
/// TODO
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserCountry {
  pub code: String, // TODO: Enum
  pub name: String,
  //
  // Undocumented
  //
  pub is_in_europe: Option<bool>,
}

///
/// TODO
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserAuth {
  pub method: String,
  pub scopes: Vec<String>,
  pub oauth_meta: Option<SimpleMap>,
}

///
/// https://developers.coinbase.com/api/v2#accounts
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  #[serde(rename = "type")]
  pub kind: AccountType,
  pub name: String,
  pub balance: Money,
  pub currency: AccountCurrency,
  pub primary: bool,
}

///
/// https://developers.coinbase.com/api/v2#addresses
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Address {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  pub name: Option<String>,
  pub address: String,
  pub network: String,
  //
  // Undocumented
  //
  pub uri_scheme: Option<String>,
  pub warning_title: Option<String>,
  pub warning_details: Option<String>,
  pub callback_url: Option<String>,
}

///
/// https://developers.coinbase.com/api/v2#notifications
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  #[serde(rename = "type")]
  pub kind: String, // TODO: Enum
  pub data: ValueMap,
  pub user: ResourceRef,
  pub account: ResourceRef,
  pub delivery_attempts: usize,
}

///
/// https://developers.coinbase.com/api/v2#payment-method
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaymentMethod {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  #[serde(rename = "type")]
  pub kind: PaymentMethodType,
  pub name: String,
  pub currency: String,
  pub primary_buy: bool,
  pub primary_sell: bool,
  pub allow_buy: bool,
  pub allow_sell: bool,
  pub allow_deposit: bool,
  pub allow_withdraw: bool,
  pub instant_buy: bool,
  pub instant_sell: bool,
  // pub fiat_account: Option<Account>,
}

///
/// https://developers.coinbase.com/api/v2#transactions
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  #[serde(rename = "type")]
  pub kind: TransactionType,
  pub status: TransactionStatus,
  pub amount: Money,
  pub native_amount: Money,
  pub description: Option<String>,
  pub details: SimpleMap,
  pub instant_exchange: bool,
  pub network: Option<ValueMap>,
  pub to: Option<ValueMap>,
  pub from: Option<ValueMap>,
  pub address: Option<ValueMap>,
  pub application: Option<ValueMap>,
  //
  // Undocumented
  //
  pub buy: Option<ValueMap>,
  pub idem: Option<String>,
}

///
/// https://developers.coinbase.com/api/v2#users
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  pub name: Option<String>,
  pub username: Option<String>,
  pub profile_location: Option<String>,
  pub profile_bio: Option<String>,
  pub profile_url: Option<String>,
  pub avatar_url: String,
  //
  // scope: wallet:user:read
  //
  pub time_zone: Option<String>,       // TODO: Enum?
  pub native_currency: Option<String>, // TODO: Enum?
  pub bitcoin_unit: Option<String>,
  pub country: Option<UserCountry>,
  //
  // scope: wallet:user:email
  //
  pub email: Option<String>,
  //
  // Undocumented
  //
  pub tiers: Option<ValueMap>,
  pub state: Option<String>,
}

///
/// https://developers.coinbase.com/api/v2#buys
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Buy {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  pub status: TransferStatus,
  pub payment_method: ResourceRef,
  pub transaction: ResourceRef,
  pub amount: Money,
  pub total: Money,
  pub subtotal: Money,
  pub fee: Money,
  pub committed: bool,
  pub instant: bool,
  pub payout_at: Option<DateTime>,
  //
  // Undocumented
  //
  pub user_reference: Option<String>,
  pub unit_price: Option<Money>,
  pub hold_until: Option<DateTime>,
  pub hold_days: Option<usize>,
  pub is_first_buy: Option<bool>,
  pub hold_business_days: Option<usize>,
  pub requires_completion_step: Option<bool>,
}

///
/// https://developers.coinbase.com/api/v2#sells
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sell {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  pub status: TransferStatus,
  pub payment_method: ResourceRef,
  pub transaction: ResourceRef,
  pub amount: Money,
  pub total: Money,
  pub subtotal: Money,
  pub fee: Money,
  pub committed: bool,
  pub instant: bool,
  pub payout_at: Option<DateTime>,
}

///
/// https://developers.coinbase.com/api/v2#deposits
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deposit {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  pub status: TransferStatus,
  pub payment_method: ResourceRef,
  pub transaction: ResourceRef,
  pub amount: Money,
  pub subtotal: Money,
  pub fee: Money,
  pub committed: bool,
  pub payout_at: Option<DateTime>,
}

///
/// https://developers.coinbase.com/api/v2#withdrawals
///
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Withdrawal {
  #[serde(flatten)]
  pub meta: ResourceMeta,
  pub status: TransferStatus,
  pub payment_method: ResourceRef,
  pub transaction: ResourceRef,
  pub amount: Money,
  pub subtotal: Money,
  pub fee: Money,
  pub committed: bool,
  pub payout_at: Option<DateTime>,
}
