use core::server::Server;

const HELLO_WORLD_HTML: &str = r#"
  <!DOCTYPE html>
  <html>
  <head>
      <title>Hello, World!</title>
  </head>
  <body>
      <h1>Hello, World!</h1>
  </body>
  </html>
"#;

const ABOUT_WORLD_HTML: &str = r#"
  <!DOCTYPE html>
  <html>
  <head>
      <title>About, World!</title>
  </head>
  <body>
      <h1>About, World!</h1>
  </body>
  </html>
"#;

fn main() {
  let host = "127.0.0.1";
  let port = 4000;
  let mut server = Server::new();

  fn hello_world_handler(_: &[u8]) {
    println!("Handling Hello, World! request");
    println!("{}", HELLO_WORLD_HTML);
  }

  fn about_world_handler(_: &[u8]) {
    println!("Handling About, World! request");
    println!("{}", ABOUT_WORLD_HTML);
  }

  server.set_route("/", hello_world_handler);
  server.set_route("/about", about_world_handler);

  server.listen(host, port);
}
