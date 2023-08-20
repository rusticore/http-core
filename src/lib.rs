pub mod server {
  use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{
      atomic::{AtomicBool, Ordering},
      Arc, Mutex,
    },
    thread,
  };

  use crate::handler::Handler;
  use crate::params::Params;
  pub struct Server {
    initialized: bool,
    routes: Arc<Mutex<HashMap<String, Box<dyn Fn(&[u8]) + Send + Sync>>>>,
  }

  impl Server {
    pub fn new() -> Self {
      Server {
        initialized: true,
        routes: Arc::new(Mutex::new(HashMap::new())),
      }
    }

    fn initialize(&self) {
      if !self.initialized {
        panic!("Server instance must be initialized using Server::new() before calling listen()");
      }
    }

    pub fn routes<F>(&mut self, route: &str, handler: F)
    where
      F: Fn(&[u8]) + Send + Sync + 'static,
    {
      self.initialize();

      self
        .routes
        .lock()
        .unwrap()
        .insert(route.to_string(), Box::new(handler));
    }

    pub fn listen(&self, host: &str, port: u32) {
      self.initialize();

      let mut addr = String::new();

      if Params::is_empty(host, port) {
        panic!("An error occured, parameters need to be not empty");
      }

      if !host.is_empty() && !Params::is_valid_local_host(host) {
        panic!("An error occured, parameters need to be valid local host");
      }

      if !port.to_string().is_empty() && !Params::is_valid_port(port) {
        panic!("An error occured, parameters need to be valid local port");
      }

      addr.push_str(&host);
      addr.push(':');
      addr.push_str(&port.to_string());

      let term = Arc::new(AtomicBool::new(false));

      self::Handler::handle_exit_process(Arc::clone(&term).clone());

      println!("App is listening on {}", &addr);

      let listener = TcpListener::bind(&addr).expect("An error occured while listening app");

      for stream in listener.incoming() {
        if term.load(Ordering::SeqCst) {
          println!("Exiting thread...");

          break;
        }

        match stream {
          Ok(tcp_stream) => {
            let routes_clone = Arc::clone(&self.routes);

            thread::spawn(move || {
              let routes_lock = routes_clone.lock().unwrap();
              let router = routes_lock.get("/").unwrap_or_else(|| {
                panic!("An error occurred, no handler found for default route");
              });

              Handler::handle_connection(tcp_stream, router);
            });
          }
          Err(e) => panic!("{}", e),
        };
      }
    }
  }
}

pub mod handler {
  use std::{
    io::Read,
    net::TcpStream,
    process,
    sync::{
      atomic::{AtomicBool, Ordering},
      Arc,
    },
  };

  pub struct Handler {}

  impl Handler {
    pub fn handle_connection(mut tcp_stream: TcpStream, router: &Box<dyn Fn(&[u8]) + Send + Sync>) {
      loop {
        let mut buffer = [0; 1028];

        match tcp_stream.read(&mut buffer) {
          Ok(stream_count) => {
            if stream_count == 0 {
              break;
            }

            router(&buffer[0..stream_count]);
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
