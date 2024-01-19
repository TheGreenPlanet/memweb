use crate::memory;
use shared::{process::{*}, protocol::*};
use std::{io::Error, net::TcpStream};
use tungstenite::{
    Message, WebSocket,
};

use memory::Memory;

enum ClientServerStateFlow {
    NewBorn,
    Connected,
    ReceivedProcesses,
    TargetPID,
    Unknown,
}

pub struct ClientSession {
    pub websocket: WebSocket<TcpStream>,
    state: ClientServerStateFlow,
    memory: Memory,
}

impl ClientSession {
    pub fn new(websocket: WebSocket<TcpStream>) -> Self {
        Self {
            websocket,
            state: ClientServerStateFlow::NewBorn,
            memory: Memory::new(-1),
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
            },
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
            },
            Some(PacketType::TargetPID) => {
                let packet = C2STargetPidPacket::parse(&packet_data);
                self.set_target_pid(packet.target_pid);

                match get_regions(packet.target_pid) {
                    Ok(regions) => {
                        self.websocket
                            .write_message(Message::Binary(
                                S2CTargetPidRegionsPacket::out_bytes(regions),
                            ))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error),
                }
            },
            Some(PacketType::SendProcesses) => {
                match get_running_processes() {
                    Ok(processes) => {
                        self.websocket
                            .write_message(Message::Binary(
                                S2CSendProcessesPacket::out_bytes(processes),
                            ))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error),
                }
            },
            _ => println!("Unknown packet type"),
        };
    }

    #[cfg(not(feature = "fake_read_write"))]
    fn set_target_pid(&mut self, pid: i32) {
        self.state = ClientServerStateFlow::TargetPID;
        self.memory.pid = pid;
    }

    #[cfg(feature = "fake_read_write")]
    fn set_target_pid(&mut self, pid: i32) {
        println!("Target Pid: {}", pid);
        self.state = ClientServerStateFlow::TargetPID;
        self.memory.pid = pid;
    }
}
