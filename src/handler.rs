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

          let request = &buffer[0..stream_count];
          router(request);

          println!("{}", Handler::get_request_path(request));
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

  pub fn get_request_path(request: &[u8]) -> String {
    let request_str = String::from_utf8_lossy(request);
    let request_lines: Vec<&str> = request_str.lines().collect();

    let request_path = request_lines
      .first()
      .unwrap_or(&"")
      .split_whitespace()
      .nth(1)
      .unwrap_or("");

    request_path.to_owned().to_string()
  }
}
