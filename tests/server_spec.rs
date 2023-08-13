use core;

#[test]
fn server_listen() {
  let result = core::server::Server::listen("127.0.0.1", 4000);

  assert_eq!(result, "127.0.0.1:4000");
}
