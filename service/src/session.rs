use crate::memory;
use shared::{process::*, protocol::*};
use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

use memory::Memory;

#[allow(dead_code)]
enum ClientSessionState {
    NewBorn,
    Connected,
    ReceivedProcesses,
    TargetPID,
    Unknown,
}

pub struct ClientSession {
    stream: TcpStream,
    state: ClientSessionState,
    memory: Memory,
}

impl ClientSession {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: ClientSessionState::NewBorn,
            memory: Memory::new(-1),
        }
    }

    fn error_response(&mut self, error: Error) {
        self.stream.write_all(error.to_string().as_bytes()).unwrap();
    }

    pub fn handle_message(&mut self) {
        let mut buffer = [0; 1024];
        let size = self.stream.read(&mut buffer).unwrap();
        let msg = buffer[..size].to_vec();

        match PacketType::from_u8(msg[0]) {
            Some(PacketType::Read) => {
                let packet = C2SReadMemoryPacket::parse(&msg);

                match self.memory.read(packet.address, packet.size as usize) {
                    Ok(result) => {
                        self.stream
                            .write_all(&S2CReadMemoryPacketResponse::out_bytes(result))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error),
                }
            }
            Some(PacketType::Write) => {
                let packet = C2SWriteMemoryPacket::parse(&msg);

                match self.memory.write(packet.address, &packet.bytes) {
                    Ok(result) => {
                        self.stream
                            .write_all(&S2CWriteMemoryPacketResponse::out_bytes(result as u64))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error),
                }
            }
            Some(PacketType::TargetPID) => {
                let packet = C2STargetPidPacket::parse(&msg);
                self.set_target_pid(packet.target_pid);

                match get_regions(packet.target_pid) {
                    Ok(regions) => {
                        self.stream
                            .write_all(&S2CTargetPidRegionsPacket::out_bytes(regions))
                            .unwrap();
                    }
                    Err(error) => self.error_response(error),
                }
            }
            Some(PacketType::SendProcesses) => match get_running_processes() {
                Ok(processes) => {
                    self.stream
                        .write_all(&S2CSendProcessesPacket::out_bytes(processes))
                        .unwrap();
                }
                Err(error) => self.error_response(error),
            },
            _ => println!("Unknown packet type"),
        };
    }

    #[cfg(not(feature = "fake_read_write"))]
    fn set_target_pid(&mut self, pid: i32) {
        self.state = ClientSessionState::TargetPID;
        self.memory.pid = pid;
    }

    #[cfg(feature = "fake_read_write")]
    fn set_target_pid(&mut self, pid: i32) {
        println!("Target Pid: {}", pid);
        self.state = ClientSessionState::TargetPID;
        self.memory.pid = pid;
    }
}
