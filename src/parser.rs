use std::net::TcpStream;

use tungstenite::WebSocket;


pub enum ParserType {
    
}

pub struct ClientSession {
    websocket: &WebSocket<TcpStream>
}

impl ClientSession {
    pub fn new(websocket: &WebSocket<TcpStream>) -> ClientSession {
        ClientSession { websocket }
    }
}

pub fn parser() -> ParserType {

}