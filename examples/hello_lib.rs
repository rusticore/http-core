use core::server::Server;

fn main() {
  let host = "127.0.0.1";
  let port = 4000;

  Server::listen(host, port);
}
