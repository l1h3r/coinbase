#[macro_export]
macro_rules! json {
  () => {
    compile_error!("Invalid JSON");
  };
  ($($key:ident : $val:expr),* $(,)*) => {{
    use ::std::collections::HashMap;

    let mut map = HashMap::new();

    $(
      map.insert(stringify!($key), $val);
    )*

    serde_json::to_string(&map).expect("Invalid JSON")
  }};
}

macro_rules! url {
  ($path:expr) => {
    ::reqwest::Url::parse(&[$crate::client::ENDPOINT, $path].join("")).unwrap()
  };
  ($path:expr, $params:expr) => {
    ::reqwest::Url::parse_with_params(&[$crate::client::ENDPOINT, $path].join(""), $params).unwrap()
  };
}
