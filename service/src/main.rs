use log::info;
use std::{env, net::TcpListener};

mod memory;
mod session;

fn main() {
    let _ = env_logger::init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8069".to_string());

    let listener = TcpListener::bind(addr.clone()).unwrap();
    info!("Listening on: {}", addr);

      // Accept a single connection
    if let Ok((stream, _addr)) = listener.accept() {
        let mut session = session::ClientSession::new(stream);
        session.handle_message();
    }
}
