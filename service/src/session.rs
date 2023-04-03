use std::{io::Error, net::TcpStream};
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message, WebSocket,
};

use crate::memory;

enum ClientServerStateFlow {
    NewBorn,
    Connected,
    ReceivedProcesses,
    TargetPID,
    Unknown,
}

pub struct ClientSession {
    websocket: WebSocket<TcpStream>,
    state: ClientServerStateFlow,
    memory: memory::Memory,
}

enum PacketType {
    Read = 0,
    Write = 1,
    TargetPID = 2,
    SendProcesses = 3,
    Error = 4,
}

impl PacketType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Read),
            1 => Some(Self::Write),
            2 => Some(Self::TargetPID),
            3 => Some(Self::SendProcesses),
            4 => Some(Self::Error),
            _ => None,
        }
    }
}

impl ClientSession {
    pub fn new(websocket: WebSocket<TcpStream>) -> Self {
        Self {
            websocket,
            memory: memory::Memory::new(-1),
            state: ClientServerStateFlow::NewBorn,
        }
    }

    pub fn message_handler(&mut self, msg: Message) {
        if !valid_packet(&msg) {
            return;
        }

        let data = msg.into_data();
        match PacketType::from_u8(data[0]) {
            Some(PacketType::Read) => {
                println!("Read memory");
                let address = u64::from_be_bytes([
                    data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
                ]);
                let bytes = u64::from_be_bytes([
                    data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16],
                ]);
                match self.memory.read(address, bytes as usize) {
                    Ok(data) => {
                        self.websocket
                            .write_message(response_packet(PacketType::Read, data))
                            .unwrap();
                        return;
                    }
                    Err(error) => {
                        self.websocket
                            .write_message(response_packet_error(error))
                            .unwrap();
                        return;
                    }
                }
            }
            Some(PacketType::Write) => {
                println!("Write memory");
                let address = u64::from_be_bytes([
                    data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
                ]);
                match self.memory.write(address, &data[2..]) {
                    Ok(()) => {
                        self.websocket
                            .write_message(response_packet(PacketType::Write, data))
                            .unwrap();
                        return;
                    }
                    Err(error) => {
                        self.websocket
                            .write_message(response_packet_error(error))
                            .unwrap();
                        return;
                    }
                }
            }
            Some(PacketType::TargetPID) => {
                println!("Target PID");
                self.set_target_pid(i32::from_le_bytes([data[1], data[2], data[3], data[4]]));
            }
            Some(PacketType::SendProcesses) => {
                println!("Send Processes");
            }
            _ => println!("Unknown packet type"),
        };
    }

    pub fn websocket(&mut self) -> &mut WebSocket<TcpStream> {
        &mut self.websocket
    }

    fn set_target_pid(&mut self, pid: i32) {
        self.state = ClientServerStateFlow::TargetPID;
        self.memory.pid(pid);
    }
}

fn valid_packet(msg: &Message) -> bool {
    const MIN_PACKET_LEN: usize = 1 + 8 + 1;
    return msg.len() >= MIN_PACKET_LEN;
}

fn response_packet(_type: PacketType, data: Vec<u8>) -> Message {
    // TODO: avoid copy
    let mut payload = data.clone();
    let byte_to_add = 0;
    payload.insert(0, byte_to_add);

    Message::binary(payload)
}

fn response_packet_error(error: Error) -> Message {
    Message::text(error.to_string())
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