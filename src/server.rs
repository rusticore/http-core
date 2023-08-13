use crate::utils::params::Params;

pub struct Server {}

impl Server {
  pub fn new() {}
  pub fn listen(host: &str, port: u32) -> String {
    let mut addr = String::new();

    if Params::is_empty(host, port) {
      panic!("An error occured, parameters need to be not empty");
    }

    addr.push_str(&host);
    addr.push(':');
    addr.push_str(&port.to_string());

    format!("{}", addr)
  }
}
