use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response}, Message,
};
use std::env;

mod session;
mod memory;

fn main() {
    let _ = env_logger::init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server = TcpListener::bind(addr).unwrap();
    // info!("Listening on: {}", addr);
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MemWebServiceHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let websocket = accept_hdr(stream.unwrap(), callback).unwrap();
            let mut session = session::ClientSession::new(websocket);

            loop {
                let msg = session.websocket().read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    session.message_handler(msg);
                }
            }
        });
    }
}