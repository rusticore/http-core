use crate::handler::Handler;
use crate::validator;
use std::{
  collections::HashMap,
  net::TcpListener,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
  thread,
};

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

  pub fn set_route<F>(&mut self, route: &str, handler: F)
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
    validator::validate_host_and_port(host, port);
    let addr = format!("{}:{}", host, port);
    let term = Arc::new(AtomicBool::new(false));

    self::Handler::handle_exit_process(Arc::clone(&term).clone());

    let listener = TcpListener::bind(&addr).unwrap();
    match listener.accept() {
      Ok((_socket, _addr)) => println!("App is listening on {}", &addr),
      Err(e) => println!("An error occured while listening app with error, {}", e),
    }

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
