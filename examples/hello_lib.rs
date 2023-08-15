use core::server::Server;

fn main() {
  let host = "127.0.0.1";
  let port = 4000;
  let server = Server::new();

  server.listen(host, port);
}
