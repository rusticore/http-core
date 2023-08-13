use regex::Regex;

pub struct Params {}

impl Params {
  pub fn is_empty(host: &str, port: u32) -> bool {
    let is_host_empty = host.is_empty();
    let is_port_empty = port == 0;

    is_host_empty && is_port_empty
  }

  pub fn is_valid_local_host(host: &str) -> bool {
    let pattern = r"^(localhost|127\.0\.0\.1)$";

    match Regex::new(pattern) {
      Ok(re) => re.is_match(host),
      Err(_) => return false,
    }
  }
}
