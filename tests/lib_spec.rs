use core::params::Params;

#[test]
fn is_empty() {
  let host = "127.0.0.1";
  let port = 4000;
  let result = Params::is_empty(host, port);

  assert_eq!(result, false);
}

#[test]
fn is_valid_local_host() {
  let host = "127.0.0.1";
  let result = Params::is_valid_local_host(host);

  assert_eq!(result, true);
}
