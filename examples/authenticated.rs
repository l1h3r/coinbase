use coinbase::prelude::*;
use std::env::var;

fn main() {
  let key = var("COINBASE_API_KEY").unwrap_or_else(|_| {
    panic!("You must set `COINBASE_API_KEY`");
  });

  let secret = var("COINBASE_API_SECRET").unwrap_or_else(|_| {
    panic!("You must set `COINBASE_API_SECRET`");
  });

  let client = Client::private(&key, &secret);

  let user_auth = client.current_user_auth().unwrap();
  let user_data = client.current_user().unwrap();

  println!("User Auth:\n{:#?}\n", user_auth);
  println!("User Data:\n{:#?}\n", user_data);
}
