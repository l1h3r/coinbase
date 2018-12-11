use coinbase::prelude::*;

fn main() {
  let client = Client::new();
  let response = client.spot_price("BTC", "USD").unwrap();

  println!("BTC-USD Spot Price: {:?}", response);
}
