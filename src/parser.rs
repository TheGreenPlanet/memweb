use std::net::TcpStream;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message, WebSocket,
};


enum ClientServerStateFlow {
    NewBorn,
    Connected,
    ReceivedProcesses,
    TargetPID,
    Unknown,
}

pub struct ClientSession {
    websocket: WebSocket<TcpStream>,
    target_pid: i32,
    state: ClientServerStateFlow,
}

impl ClientSession {
    pub fn new(websocket: WebSocket<TcpStream>) -> Self {
        Self {
            websocket,
            target_pid: -1,
            state: ClientServerStateFlow::NewBorn,
        }
    }

    pub fn message_handler(&mut self, msg: Message) {
        if !valid_packet(&msg) {
            return;
        }

        let data = msg.into_data();
        match data[0] {
            0 => println!("Read memory"),
            1 => println!("Write memory"),
            2 => {
                println!("Target PID");
                self.state = ClientServerStateFlow::TargetPID;
            },
            _ => println!("Unknown packet type"),
        };
    }

    pub fn websocket(&mut self) -> &mut WebSocket<TcpStream> {
        &mut self.websocket
    }
}

fn valid_packet(msg: &Message) -> bool {
    const MIN_PACKET_LEN: usize = 1 + 8 + 1;
    return msg.len() >= MIN_PACKET_LEN;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_packet() {
        // create a message with a size of 9
        let short_msg = Message::binary(vec![0; 9]);
        assert_eq!(valid_packet(&short_msg), false);

        let edge_msg = Message::binary(vec![0; 10]);
        assert_eq!(valid_packet(&edge_msg), true);

        let large_msg = Message::binary(vec![0; 100]);
        assert_eq!(valid_packet(&large_msg), true);
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
