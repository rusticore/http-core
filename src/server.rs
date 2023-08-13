use crate::utils::params::Params;

pub struct Server {}

impl Server {
  pub fn new() {}
  pub fn listen(host: &str, port: u32) -> String {
    let mut addr = String::new();

    if Params::is_empty(host, port) {
      panic!("An error occured, parameters need to be not empty");
    }

    if !Params::is_valid_local_host(host) {
      panic!("An error occured, parameters need to be valid local host");
    }

    addr.push_str(&host);
    addr.push(':');
    addr.push_str(&port.to_string());

    format!("{}", addr)
  }
}
