# Coinbase

Client library for the Coinbase Wallet API.

[API Reference](https://developers.coinbase.com/api/v2)

## Usage:

Example using the API to fetch BTC-USD buy/sell prices

```rust
extern crate coinbase;

use coinbase::prelude::*;

fn main() {
  let client = Client::new();

  let buy = client
    .buy_price("BTC", "USD")
    .expect("Failed to fetch BTC-USD buy price");

  let sell = client
    .sell_price("BTC", "USD")
    .expect("Failed to fetch BTC-USD sell price");

  println!("BTC-USD Buy Price:  {:#?}", buy);
  println!("BTC-USD Sell Price: {:#?}", sell);
}
```

## TODO:

- [ ] Pagination - https://developers.coinbase.com/api/v2#pagination
- [ ] Expanded Queries - https://developers.coinbase.com/api/v2#expanding-resources
