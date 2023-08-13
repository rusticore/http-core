pub struct Params {}

impl Params {
  pub fn is_empty(host: &str, port: u32) -> bool {
    let is_host_empty = host.is_empty();
    let is_port_empty = port == 0;

    is_host_empty && is_port_empty
  }
}
