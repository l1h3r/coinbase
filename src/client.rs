use hmac::Hmac;
use hmac::Mac;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Body;
use reqwest::Client as Http;
use reqwest::Method;
use reqwest::Request;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_str;
use serde_json::to_string;
use sha2::Sha256;

use crate::error::Error;
use crate::types::*;

type HmacSha = Hmac<Sha256>;

const ENDPOINT: &str = "https://api.coinbase.com/v2/";
const U_AGENT: &str = concat!("coinbase/rs/", env!("CARGO_PKG_VERSION"));
const VERSION: &str = "2019-11-15";

#[derive(Debug)]
pub struct Client {
  http: Http,
  key: String,
  secret: String,
  pub language: Language,
  pub uagent: &'static str,
  pub version: &'static str,
}

impl Default for Client {
  fn default() -> Self {
    Self {
      http: Http::new(),
      key: Default::default(),
      secret: Default::default(),
      language: Default::default(),
      uagent: U_AGENT,
      version: VERSION,
    }
  }
}

impl Client {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn private(key: &str, secret: &str) -> Self {
    Self {
      key: key.to_owned(),
      secret: secret.to_owned(),
      ..Default::default()
    }
  }

  //
  // Public
  //

  /// Get the API server time.
  ///
  /// https://developers.coinbase.com/api/v2#get-current-time
  pub fn time(&self) -> CBResult<Time> {
    self.get("time")
  }

  /// List known currencies.
  ///
  /// https://developers.coinbase.com/api/v2#get-currencies
  pub fn currencies(&self) -> CBResult<Vec<Currency>> {
    self.get("currencies")
  }

  /// Get current exchange rates.
  ///
  /// https://developers.coinbase.com/api/v2#get-exchange-rates
  pub fn rates(&self, currency: Option<&str>) -> CBResult<Rates> {
    self.get(&format!("exchange-rates?currency={}", currency.unwrap_or("USD")))
  }

  /// Get the total price to buy one unit of any supported currency pairs.
  ///
  /// https://developers.coinbase.com/api/v2#get-buy-price
  pub fn buy_price(&self, currency: &str, other: &str) -> CBResult<Money> {
    self.get(&format!("prices/{}-{}/buy", currency, other))
  }

  /// Get the total price to sell one unit of any supported currency pairs.
  ///
  /// https://developers.coinbase.com/api/v2#get-sell-price
  pub fn sell_price(&self, currency: &str, other: &str) -> CBResult<Money> {
    self.get(&format!("prices/{}-{}/sell", currency, other))
  }

  /// Get the current market price for supported currency pairs.
  ///
  /// https://developers.coinbase.com/api/v2#get-spot-price
  pub fn spot_price(&self, currency: &str, other: &str) -> CBResult<Money> {
    self.get(&format!("prices/{}-{}/spot", currency, other))
  }

  /// Get the historic market price for supported currency pairs.
  ///
  /// https://developers.coinbase.com/api/v2#get-spot-price
  pub fn historic_spot_price(&self, currency: &str, other: &str, date: &str) -> CBResult<Money> {
    self.get(&format!("prices/{}-{}/spot?date={}", currency, other, date))
  }

  //
  // Notifications
  //

  /// Lists notifications where the current user was the subscriber
  ///
  /// https://developers.coinbase.com/api/v2#list-notifications
  pub fn list_notifications(&self) -> CBResult<Vec<Notification>> {
    self.get("notifications")
  }

  /// Show a notification for which the current user was a subscriber.
  ///
  /// https://developers.coinbase.com/api/v2#show-a-notification
  pub fn get_notification(&self, notification: &str) -> CBResult<Notification> {
    self.get(&format!("notifications/{}", notification))
  }

  //
  // Users
  //

  /// Get any user's public information with their ID.
  ///
  /// https://developers.coinbase.com/api/v2#show-a-user
  pub fn get_user(&self, user: &str) -> CBResult<User> {
    self.get(&format!("users/{}", user))
  }

  /// Get current user's public information.
  ///
  /// https://developers.coinbase.com/api/v2#show-current-user
  pub fn current_user(&self) -> CBResult<User> {
    self.get("user")
  }

  /// Get current user's authorization information including granted
  /// scopes and send limits when using OAuth2 authentication.
  ///
  /// https://developers.coinbase.com/api/v2#show-authorization-information
  pub fn current_user_auth(&self) -> CBResult<UserAuth> {
    self.get("user/auth")
  }

  /// Modify current user and their preferences.
  ///
  /// https://developers.coinbase.com/api/v2#update-current-user
  pub fn update_user<D: Serialize>(&self, data: D) -> CBResult<User> {
    self.put("user", self.serialize(data)?)
  }

  //
  // Accounts
  //

  /// Lists current user's accounts to which the authentication method has access to.
  ///
  /// https://developers.coinbase.com/api/v2#list-accounts
  pub fn list_accounts(&self) -> CBResult<Vec<Account>> {
    self.get("accounts")
  }

  /// Show current user's account.
  ///
  /// https://developers.coinbase.com/api/v2#show-an-account
  pub fn get_account(&self, account: &str) -> CBResult<Account> {
    self.get(&format!("accounts/{}", account))
  }

  /// Modifies user's account.
  ///
  /// https://developers.coinbase.com/api/v2#update-account
  pub fn update_account(&self, account: &str, name: &str) -> CBResult<Account> {
    self.put(&format!("accounts/{}", account), json!(name: name))
  }

  /// Removes user's account.
  ///
  /// https://developers.coinbase.com/api/v2#delete-account
  pub fn delete_account(&self, account: &str) -> CBResult<()> {
    self.delete(&format!("accounts/{}", account))
  }

  /// Promote an account as primary account.
  ///
  /// https://developers.coinbase.com/api/v2#set-account-as-primary
  pub fn set_primary_account(&self, account: &str) -> CBResult<Account> {
    self.post(&format!("accounts/{}/primary", account), "")
  }

  //
  // Addresses
  //

  /// Lists addresses for an account.
  ///
  /// https://developers.coinbase.com/api/v2#list-addresses
  pub fn list_addresses(&self, account: &str) -> CBResult<Vec<Address>> {
    self.get(&format!("accounts/{}/addresses", account))
  }

  /// Show an individual address for an account
  ///
  /// https://developers.coinbase.com/api/v2#show-addresss
  pub fn get_address(&self, account: &str, address: &str) -> CBResult<Address> {
    self.get(&format!("accounts/{}/addresses/{}", account, address))
  }

  /// Creates a new address for an account.
  ///
  /// https://developers.coinbase.com/api/v2#create-address
  pub fn create_address(&self, account: &str, name: Option<&str>) -> CBResult<Address> {
    let body: String = name.map(|name| json!(name: name)).unwrap_or_default();

    self.post(&format!("accounts/{}/addresses", account), body)
  }

  /// List transactions that have been sent to a specific address.
  ///
  /// https://developers.coinbase.com/api/v2#list-address39s-transactions
  pub fn list_address_transactions(&self, account: &str, address: &str) -> CBResult<Transaction> {
    self.get(&format!("accounts/{}/addresses/{}/transactions", account, address))
  }

  //
  // Transactions
  //

  /// Lists account's transactions
  ///
  /// https://developers.coinbase.com/api/v2#list-transactions
  pub fn list_transactions(&self, account: &str) -> CBResult<Vec<Transaction>> {
    self.get(&format!("accounts/{}/transactions", account))
  }

  /// Show an individual transaction for an account
  ///
  /// https://developers.coinbase.com/api/v2#show-a-transaction
  pub fn get_transaction(&self, account: &str, transaction: &str) -> CBResult<Transaction> {
    self.get(&format!("accounts/{}/transactions/{}", account, transaction))
  }

  /// Creates a new transaction for an account.
  ///
  /// Send funds to a bitcoin address, bitcoin cash address, litecoin address, ethereum address, or email address
  /// https://developers.coinbase.com/api/v2#send-money
  ///
  /// Transfer bitcoin, bitcoin cash, litecoin or ethereum between two of a user's accounts
  /// https://developers.coinbase.com/api/v2#transfer-money-between-accounts
  ///
  /// Requests money from an email address.
  /// https://developers.coinbase.com/api/v2#request-money
  pub fn create_transaction<D: Serialize>(&self, account: &str, data: D) -> CBResult<Transaction> {
    self.post(&format!("accounts/{}/transactions", account), self.serialize(data)?)
  }

  /// Lets the recipient of a money request complete the request by sending money to the user who requested the money.
  ///
  /// https://developers.coinbase.com/api/v2#complete-request-money
  pub fn complete_request(&self, account: &str, transaction: &str) -> CBResult<()> {
    self.post(
      &format!("accounts/{}/transactions/{}/complete", account, transaction),
      "",
    )
  }

  /// Lets the user resend a money request.
  ///
  /// https://developers.coinbase.com/api/v2#re-send-request-money
  pub fn resend_request(&self, account: &str, transaction: &str) -> CBResult<()> {
    self.post(&format!("accounts/{}/transactions/{}/resend", account, transaction), "")
  }

  /// Lets a user cancel a money request
  ///
  /// https://developers.coinbase.com/api/v2#cancel-request-money
  pub fn cancel_request(&self, account: &str, transaction: &str) -> CBResult<()> {
    self.delete(&format!("accounts/{}/transactions/{}", account, transaction))
  }

  //
  // Buys
  //

  /// Lists buys for an account.
  ///
  /// https://developers.coinbase.com/api/v2#list-buys
  pub fn list_buys(&self, account: &str) -> CBResult<Vec<Buy>> {
    self.get(&format!("accounts/{}/buys", account))
  }

  /// Show an individual buy.
  ///
  /// https://developers.coinbase.com/api/v2#show-a-buy
  pub fn get_buy(&self, account: &str, buy: &str) -> CBResult<Buy> {
    self.get(&format!("accounts/{}/buys/{}", account, buy))
  }

  /// Buys a user-defined amount of bitcoin, bitcoin cash, litecoin or ethereum.
  ///
  /// https://developers.coinbase.com/api/v2#place-buy-order
  pub fn create_buy<D: Serialize>(&self, account: &str, data: D) -> CBResult<Buy> {
    self.post(&format!("accounts/{}/buys", account), self.serialize(data)?)
  }

  /// Completes a buy that is created in `commit: false` state.
  ///
  /// https://developers.coinbase.com/api/v2#commit-a-buy
  pub fn commit_buy(&self, account: &str, buy: &str) -> CBResult<Buy> {
    self.post(&format!("accounts/{}/buys/{}/commit", account, buy), "")
  }

  //
  // Sells
  //

  /// Lists sells for an account.
  ///
  /// https://developers.coinbase.com/api/v2#list-sells
  pub fn list_sells(&self, account: &str) -> CBResult<Vec<Sell>> {
    self.get(&format!("accounts/{}/sells", account))
  }

  /// Show an individual sell.
  ///
  /// https://developers.coinbase.com/api/v2#show-a-sell
  pub fn get_sell(&self, account: &str, sell: &str) -> CBResult<Sell> {
    self.get(&format!("accounts/{}/sells/{}", account, sell))
  }

  /// Sells a user-defined amount of bitcoin, bitcoin cash, litecoin or ethereum.
  ///
  /// https://developers.coinbase.com/api/v2#place-sell-order
  pub fn create_sell<D: Serialize>(&self, account: &str, data: D) -> CBResult<Sell> {
    self.post(&format!("accounts/{}/sells", account), self.serialize(data)?)
  }

  /// Completes a sell that is created in `commit: false` state.
  ///
  /// https://developers.coinbase.com/api/v2#commit-a-sell
  pub fn commit_sell(&self, account: &str, sell: &str) -> CBResult<Buy> {
    self.post(&format!("accounts/{}/sells/{}/commit", account, sell), "")
  }

  //
  // Deposits
  //

  /// Lists deposits for an account.
  ///
  /// https://developers.coinbase.com/api/v2#list-deposits
  pub fn list_deposits(&self, account: &str) -> CBResult<Vec<Deposit>> {
    self.get(&format!("accounts/{}/deposits", account))
  }

  /// Show an individual deposit.
  ///
  /// https://developers.coinbase.com/api/v2#show-a-deposit
  pub fn get_deposit(&self, account: &str, deposit: &str) -> CBResult<Deposit> {
    self.get(&format!("accounts/{}/deposits/{}", account, deposit))
  }

  /// Deposits user-defined amount of funds to a fiat account.
  ///
  /// https://developers.coinbase.com/api/v2#deposit-funds
  pub fn create_deposit<D: Serialize>(&self, account: &str, data: D) -> CBResult<Deposit> {
    self.post(&format!("accounts/{}/deposits", account), self.serialize(data)?)
  }

  /// Completes a deposit that is created in `commit: false` state.
  ///
  /// https://developers.coinbase.com/api/v2#commit-a-deposit
  pub fn commit_deposit(&self, account: &str, deposit: &str) -> CBResult<Deposit> {
    self.post(&format!("accounts/{}/deposits/{}/commit", account, deposit), "")
  }

  //
  // Withdrawals
  //

  /// Lists withdrawals for an accounts.
  ///
  /// https://developers.coinbase.com/api/v2#list-withdrawals
  pub fn list_withdrawals(&self, account: &str) -> CBResult<Vec<Withdrawal>> {
    self.get(&format!("accounts/{}/withdrawals", account))
  }

  /// Show an individual withdrawal.
  ///
  /// https://developers.coinbase.com/api/v2#show-a-withdrawal
  pub fn get_withdrawal(&self, account: &str, withdrawal: &str) -> CBResult<Withdrawal> {
    self.get(&format!("accounts/{}/withdrawals/{}", account, withdrawal))
  }

  /// Withdraws user-defined amount of funds from a fiat account.
  ///
  /// https://developers.coinbase.com/api/v2#withdraw-funds
  pub fn create_withdrawal<D: Serialize>(&self, account: &str, data: D) -> CBResult<Withdrawal> {
    self.post(&format!("accounts/{}/withdrawals", account), self.serialize(data)?)
  }

  /// Completes a withdrawal that is created in `commit: false` state.
  ///
  /// https://developers.coinbase.com/api/v2#commit-a-withdrawal
  pub fn commit_withdrawal(&self, account: &str, withdrawal: &str) -> CBResult<Withdrawal> {
    self.post(&format!("accounts/{}/withdrawals/{}/commit", account, withdrawal), "")
  }

  //
  // Payment Methods
  //

  /// Lists current user's payment methods.
  ///
  /// https://developers.coinbase.com/api/v2#list-payment-methods
  pub fn list_payment_methods(&self) -> CBResult<Vec<PaymentMethod>> {
    self.get("payment-methods")
  }

  /// Show current user's payment method.
  ///
  /// https://developers.coinbase.com/api/v2#show-a-payment-method
  pub fn get_payment_method(&self, payment_method: &str) -> CBResult<PaymentMethod> {
    self.get(&format!("payment-methods/{}", payment_method))
  }

  //
  // Private
  //

  fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
    self.request(Method::GET, url!(path), String::new())
  }

  fn post<T: DeserializeOwned, B: Into<String>>(&self, path: &str, body: B) -> Result<T, Error> {
    self.request(Method::POST, url!(path), body.into())
  }

  fn put<T: DeserializeOwned, B: Into<String>>(&self, path: &str, body: B) -> Result<T, Error> {
    self.request(Method::PUT, url!(path), body.into())
  }

  fn delete<T>(&self, _path: &str) -> Result<T, Error> {
    unimplemented!();
  }

  fn serialize<D: Serialize>(&self, data: D) -> Result<String, Error> {
    Ok(to_string(&data)?)
  }

  fn request<T: DeserializeOwned>(&self, method: Method, url: Url, body: String) -> Result<T, Error> {
    let auth: HeaderMap = self.auth_headers(&method, &url, &body)?;

    let request: Request = self
      .http
      .request(method, url)
      .header("Accept", "application/json")
      .header("Accept-Language", self.language.to_string())
      .header("Content-Type", "application/json")
      .header("User-Agent", self.uagent)
      .header("CB-VERSION", self.version)
      .headers(auth)
      .body(Body::from(body))
      .build()?;

    let data: String = self.http.execute(request)?.text()?;

    from_str(&data).map_err(|error| Error::JSON {
      error,
      data: Some(data),
    })
  }

  fn has_auth(&self) -> bool {
    !self.key.is_empty() && !self.secret.is_empty()
  }

  fn auth_headers(&self, method: &Method, url: &Url, body: &str) -> Result<HeaderMap, Error> {
    let mut headers: HeaderMap = HeaderMap::new();

    if self.has_auth() {
      let now: i64 = crate::timestamp();
      let key: HeaderValue = self.auth_header(&self.key)?;
      let timestamp: HeaderValue = self.auth_header(&format!("{}", now))?;

      let signature: HeaderValue = {
        let formatted: String = if let Some(query) = url.query() {
          format!("{}{}{}?{}{}", now, method, url.path(), query, body)
        } else {
          format!("{}{}{}{}", now, method, url.path(), body)
        };

        self
          .auth_signature(&formatted)
          .and_then(|signature| self.auth_header(&signature))?
      };

      headers.insert("CB-ACCESS-KEY", key);
      headers.insert("CB-ACCESS-SIGN", signature);
      headers.insert("CB-ACCESS-TIMESTAMP", timestamp);
    }

    Ok(headers)
  }

  fn auth_header(&self, value: &str) -> Result<HeaderValue, Error> {
    HeaderValue::from_str(value).map_err(Into::into).map(|mut value| {
      value.set_sensitive(true);
      value
    })
  }

  fn auth_signature(&self, message: &str) -> Result<String, Error> {
    let apply = |mut hmac: HmacSha| {
      hmac.input(message.as_bytes());
      hmac
    };

    let hmac: HmacSha = Hmac::new_varkey(self.secret.as_bytes()).map(apply)?;
    let signed: String = format!("{:x}", hmac.result().code());

    Ok(signed)
  }
}
