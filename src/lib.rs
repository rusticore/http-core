pub mod server {
  use std::{
    net::TcpListener,
    sync::{
      atomic::{AtomicBool, Ordering},
      Arc,
    },
    thread,
  };

  use crate::handler::Handler;
  use crate::params::Params;
  pub struct Server {
    initialized: bool,
  }

  impl Server {
    pub fn new() -> Self {
      Server { initialized: true }
    }

    fn initialize(&self) {
      if !self.initialized {
        panic!("Server instance must be initialized using Server::new() before calling listen()");
      }
    }

    pub fn listen(&self, host: &str, port: u32) {
      self.initialize();

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

      self::Handler::handle_exit_process(Arc::clone(&term).clone());

      println!("App is listening on {}", &addr);

      for stream in listener.incoming() {
        if term.load(Ordering::SeqCst) {
          println!("Exiting thread...");

          break;
        }

        match stream {
          Ok(tcp_stream) => thread::spawn(move || self::Handler::handle_connection(tcp_stream)),
          Err(e) => panic!("{}", e),
        };
      }
    }
  }
}

pub mod handler {
  use std::{
    io::{Read, Write},
    net::TcpStream,
    process,
    sync::{
      atomic::{AtomicBool, Ordering},
      Arc,
    },
  };

  pub struct Handler {}

  impl Handler {
    pub fn handle_connection(mut tcp_stream: TcpStream) {
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

    pub fn handle_exit_process(exit: Arc<AtomicBool>) {
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

  use crate::constant;

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
      let pattern = constant::HOST_REGEX;

      match Regex::new(pattern) {
        Ok(re) => re.is_match(host),
        Err(_) => false,
      }
    }

    pub fn is_valid_port(port: u32) -> bool {
      let pattern = constant::PORT_REGEX;

      match Regex::new(pattern) {
        Ok(re) => re.is_match(&port.to_string()),
        Err(_) => false,
      }
    }
  }
}

pub mod constant {
  pub const HOST_REGEX: &str = r"^(localhost|127\.0\.0\.1)$";
  pub const PORT_REGEX: &str = r"^((6553[0-5])|(655[0-2][0-9])|(65[0-4][0-9]{2})|(6[0-4][0-9]{3})|([1-5][0-9]{4})|([0-5]{0,5})|([0-9]{1,4}))$";
}
