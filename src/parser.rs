use std::net::TcpStream;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response}, Message, WebSocket,
};

enum ClientServerStateFlow {
    NewBorn,
    Connected,
    ReceivedProcesses,
    Unknown,
}

pub struct ClientSession {
    websocket: WebSocket<TcpStream>,
    target_pid: i32,
    state: ClientServerStateFlow
}

impl ClientSession {
    pub fn new(websocket: WebSocket<TcpStream>) -> Self {
        Self { websocket, target_pid: -1, state: ClientServerStateFlow::NewBorn }
    }

    pub fn message_handler(&self,  msg: &Message) {
        let data = msg.into_data(); 
        match data[0] {
            0 => println!("Read memory"),
            1 => println!("Write memory"),
            _ => println!("Unknown")
        };
    }

    pub fn websocket(&mut self) -> &mut WebSocket<TcpStream> {
        &mut self.websocket
    }
}

// enum PacketType {
//     Read,
//     Write,
//     Unknown
// }

// struct Packet {
//     _type: PacketType,
//     address: i64,
//     bytes: Vec<u8>
// }

// impl Packet {
//     pub fn new(data: &Vec<u8>) -> Self {
//         let _type = match data[0] {
//             0 => PacketType::Read,
//             1 => PacketType::Write,
//             _ => PacketType::Unknown
//         };

//         let address = i64::from_le_bytes([
//             data[1], data[2], data[3], data[4], 
//             data[5], data[6], data[7], data[8]
//         ]);

//         let bytes = data[8..].to_vec();
        
//         Self { _type, address, bytes }
//     }
// }

// fn parser(msg: &Message) -> Option<Packet> {
//     let packet = Packet::new(msg.into_data());
    
// }

// we want to be able to interpret a given datum as a read/req packet or ... 