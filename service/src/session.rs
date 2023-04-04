use crate::memory;
use shared::protocol::*;
use std::{io::Error, net::TcpStream};
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
    state: ClientServerStateFlow,
    memory: memory::Memory,
}

impl ClientSession {
    pub fn new(websocket: WebSocket<TcpStream>) -> Self {
        Self {
            websocket,
            memory: memory::Memory::new(-1),
            state: ClientServerStateFlow::NewBorn,
        }
    }

    fn error_response(&mut self, error: Error){
        self.websocket
            .write_message(Message::text(error.to_string()))
            .unwrap();
    }

    pub fn message_handler(&mut self, msg: Message) {
        if !valid_packet(&msg) {
            return;
        }

        let packet_data = msg.into_data();

        match PacketType::from_u8(packet_data[0]) {
            Some(PacketType::Read) => {
                println!("Read memory");

                let packet = C2SReadMemoryPacket::parse(&packet_data);
                match self.memory.read(packet.address, packet.size) {
                    Ok(result) => {
                        self.websocket
                            .write_message(Message::Binary(C2SReadMemoryPacketResponse::out_bytes(
                                result,
                            )))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error)
                }
            }
            Some(PacketType::Write) => {
                println!("Write memory");

                let packet = C2SWriteMemoryPacket::parse(&packet_data);
                match self.memory.write(packet.address, &packet.bytes) {
                    Ok(result) => {
                        self.websocket
                            .write_message(Message::Binary(
                                C2SWriteMemoryPacketResponse::out_bytes(result),
                            ))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error)
                }
            }
            Some(PacketType::TargetPID) => {
                println!("Target PID");

                let packet = C2STargetPidPacket::parse(&packet_data);
                self.set_target_pid(packet.target_pid as i32);
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
