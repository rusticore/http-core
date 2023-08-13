use core;

fn main() {
  let host = "127.0.0.1";
  let port = 4000;

  core::server::Server::listen(host, port);
}
