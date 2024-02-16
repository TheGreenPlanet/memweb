use log::{info, error};
use std::env;
use tokio::net::TcpListener;
use tokio::time::{self, Duration};

mod memory;
mod session;
use session::ClientSession;

#[tokio::main]
async fn main() {
    let _ = env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8069".to_string());
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Listening on: {}", addr);

    loop {
        if let Ok((stream, _addr)) = listener.accept().await {
            info!("Connection made from client: {:?}", stream.peer_addr());
            // Handle the connection in a separate asynchronous task
            tokio::spawn(async move {
                let mut session = ClientSession::new(stream);
                //let timeout_duration = Duration::from_secs(30); // 30-second timeout for example

                loop {
                    match session.handle_message().await {
                        Ok(()) => {
                            // Message was successfully processed, continue to the next message
                        },
                        Err(e) => {
                            // An error occurred while handling the message, log and break out of the loop
                            error!("Error handling message: {:?}", e);
                            break;
                        }
                    }
                }
                // If we reach here, it means either a timeout occurred or an unhandled error broke the loop
                info!("Connection closed or timed out.");
            });
        } else {
            error!("Error accepting connection");
        }
    }
}

