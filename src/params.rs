use crate::constants;
use regex::Regex;

pub fn is_empty(host: &str, port: u32) -> bool {
  let is_host_empty = host.is_empty();
  let is_port_empty = port == 0;

  match (is_host_empty, is_port_empty) {
    (true, true) => true,
    _ => false,
  }
}

pub fn is_valid_local_host(host: &str) -> bool {
  let pattern = constants::HOST_REGEX;

  match Regex::new(pattern) {
    Ok(re) => re.is_match(host),
    Err(_) => false,
  }
}

pub fn is_valid_port(port: u32) -> bool {
  let pattern = constants::PORT_REGEX;

  match Regex::new(pattern) {
    Ok(re) => re.is_match(&port.to_string()),
    Err(_) => false,
  }
}
