pub mod server {
  use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process,
    sync::{
      atomic::{AtomicBool, Ordering},
      Arc,
    },
    thread,
  };

  use crate::params::Params;
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

      let listener = TcpListener::bind(&addr).expect("An error occured while listening app");
      let term = Arc::new(AtomicBool::new(false));

      self::Server::handle_exit_process(Arc::clone(&term).clone());

      println!("App is listening on {}", &addr);

      for stream in listener.incoming() {
        if term.load(Ordering::SeqCst) {
          println!("Exiting thread...");

          break;
        }

        match stream {
          Ok(tcp_stream) => thread::spawn(move || self::Server::handle_connection(tcp_stream)),
          Err(e) => panic!("{}", e),
        };
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

    fn handle_exit_process(exit: Arc<AtomicBool>) {
      if let Err(e) = ctrlc::set_handler(move || {
        exit.store(true, Ordering::SeqCst);
        println!("Ctrl + C pressed. Exiting gracefully...");
        process::exit(0);
      }) {
        panic!("{}", e);
      }
    }
  }
}

pub mod params {
  use regex::Regex;

  pub struct Params {}

  impl Params {
    pub fn is_empty(host: &str, port: u32) -> bool {
      let is_host_empty = host.is_empty();
      let is_port_empty = port == 0;

      match (is_host_empty, is_port_empty) {
        (true, true) => true,
        _ => false,
      }
    }

    pub fn is_valid_local_host(host: &str) -> bool {
      let pattern = r"^(localhost|127\.0\.0\.1)$";

      match Regex::new(pattern) {
        Ok(re) => re.is_match(host),
        Err(_) => false,
      }
    }
  }
}
