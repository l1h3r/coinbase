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
