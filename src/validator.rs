use crate::params;

pub fn validate_host_and_port(host: &str, port: u32) {
  if params::is_empty(host, port) {
    panic!("An error occured, parameters need to be not empty");
  }

  if !host.is_empty() && !params::is_valid_local_host(host) {
    panic!("An error occured, parameters need to be valid local host");
  }

  if !port.to_string().is_empty() && !params::is_valid_port(port) {
    panic!("An error occured, parameters need to be valid local port");
  }
}
