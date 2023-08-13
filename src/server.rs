use crate::params::Params;
use std::{
  io::{Read, Write},
  net::{TcpListener, TcpStream},
  thread,
};

pub struct Server {}

impl Server {
  pub fn new() {}
  pub fn listen(host: &str, port: u32) {
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

    println!("App is listening on {}", &addr);

    match TcpListener::bind(&addr) {
      Ok(listener) => {
        for stream in listener.incoming() {
          match stream {
            Ok(tcp_stream) => thread::spawn(move || self::Server::handle_connection(tcp_stream)),
            Err(e) => panic!("{}", e),
          };
        }
      }
      Err(e) => panic!("{}", e),
    }
  }

  fn handle_connection(mut tcp_stream: TcpStream) {
    loop {
      let mut buffer = [0; 1028];

      match tcp_stream.read(&mut buffer) {
        Ok(stream_count) => {
          if stream_count == 0 {
            break;
          }

          // Get buffer request and caller properties
          match tcp_stream.write_all(&buffer[0..stream_count]) {
            Ok(()) => println!("{}", String::from_utf8_lossy(&buffer)),
            Err(e) => panic!("{}", e),
          }

          // Just get Err while writing buffer
          // if let Err(e) = tcp_stream.write_all(&buffer[0..stream_count]) {
          //   panic!("{}", e)
          // }
        }
        Err(e) => panic!("{}", e),
      }
    }
  }
}
