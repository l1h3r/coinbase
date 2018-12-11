extern crate coinbase;

use coinbase::prelude::*;
use std::thread::sleep;
use std::time::Duration;

const TIMEOUT: u64 = 200;

fn wait() {
  sleep(Duration::from_millis(TIMEOUT))
}

#[test]
fn test_time() {
  wait();

  let _ = Client::new().time().unwrap();
}

#[test]
fn test_currencies() {
  wait();

  let response = Client::new().currencies().unwrap();

  assert!(response.data.len() > 0);
}

#[test]
fn test_rates() {
  wait();

  let response = Client::new().rates(Some("EUR")).unwrap();

  assert_eq!(response.data.currency, String::from("EUR"));
  assert!(response.data.rates.contains_key("BTC"));
  assert!(response.data.rates.contains_key("ETH"));
  assert!(response.data.rates.contains_key("LTC"));
}

#[test]
fn test_buy_price() {
  wait();

  let response = Client::new().buy_price("ETH", "USD").unwrap();

  assert_eq!(response.data.currency, String::from("USD"));
  assert_eq!(response.data.base, Some(String::from("ETH")));
}

#[test]
fn test_sell_price() {
  wait();

  let response = Client::new().sell_price("LTC", "EUR").unwrap();

  assert_eq!(response.data.currency, String::from("EUR"));
  assert_eq!(response.data.base, Some(String::from("LTC")));
}

#[test]
fn test_spot_price() {
  wait();

  let response = Client::new().spot_price("BTC", "USD").unwrap();

  assert_eq!(response.data.currency, String::from("USD"));
  assert_eq!(response.data.base, Some(String::from("BTC")));
}
