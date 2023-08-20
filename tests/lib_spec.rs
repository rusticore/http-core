#[cfg(test)]
mod params {
  use core::params;

  #[test]
  fn is_empty() {
    let host = "127.0.0.1";
    let port = 4000;
    let result = params::is_empty(host, port);

    assert_eq!(result, false);
  }

  #[test]
  fn is_valid_local_host() {
    let host = "127.0.0.1";
    let result = params::is_valid_local_host(host);

    assert_eq!(result, true);
  }

  #[test]
  fn is_valid_port() {
    let port = 8080;
    let result = params::is_valid_port(port);

    assert_eq!(result, true);
  }
}
