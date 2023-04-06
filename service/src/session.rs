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

    fn error_response(&mut self, error: Error) {
        self.websocket
            .write_message(Message::text(error.to_string()))
            .unwrap();
    }

    pub fn message_handler(&mut self, msg: Message) {
        let packet_data = msg.into_data();

        match PacketType::from_u8(packet_data[0]) {
            Some(PacketType::Read) => {
                let packet = C2SReadMemoryPacket::parse(&packet_data);

                match self.memory.read(packet.address, packet.size as usize) {
                    Ok(result) => {
                        self.websocket
                            .write_message(Message::Binary(S2CReadMemoryPacketResponse::out_bytes(
                                result,
                            )))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error),
                }
            }
            Some(PacketType::Write) => {
                let packet = C2SWriteMemoryPacket::parse(&packet_data);

                match self.memory.write(packet.address, &packet.bytes) {
                    Ok(result) => {
                        self.websocket
                            .write_message(Message::Binary(
                                S2CWriteMemoryPacketResponse::out_bytes(result as u64),
                            ))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error),
                }
            }
            Some(PacketType::TargetPID) => {
                let packet = C2STargetPidPacket::parse(&packet_data);

                self.set_target_pid(packet.target_pid);
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

    #[cfg(not(feature = "fake_read_write"))]
    fn set_target_pid(&mut self, pid: i32) {
        self.state = ClientServerStateFlow::TargetPID;
        self.memory.pid(pid);
    }

    #[cfg(feature = "fake_read_write")]
    fn set_target_pid(&mut self, pid: i32) {
        println!("Target Pid: {}", pid);
        self.state = ClientServerStateFlow::TargetPID;
        self.memory.pid(pid);
    }
}
