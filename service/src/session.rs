use crate::memory;
use log::{error, info, warn};
use shared::{process::*, protocol::read_primitives::*, protocol::write_primitives::*, protocol::*};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
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

    async fn error_response(&mut self, err: io::Error) -> Result<(), io::Error> {
        let error_message = format!("Error: {}", err);
        error!("{}", error_message);
        self.stream.write_all(error_message.as_bytes()).await // Await the async write operation
    }

    pub async fn handle_message(&mut self) -> Result<(), io::Error> {
        let mut buffer = [0; 1024];
        let size = self.stream.read(&mut buffer).await?;
        if size == 0 {
            // Connection closed by the client
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Client disconnected",
            ));
        }
        let msg = buffer[..size].to_vec();

        match PacketType::from_u8(msg[0]) {
            Some(PacketType::ReadVec) => {
                let packet = RequestReadVecMemoryPacket::deserialize(&msg);

                match self.memory.read_vec(packet.address, packet.size as usize) {
                    Ok(result) => {
                        self.stream
                            .write_all(&ReceiveReadVecPacketResponse::serialize(result))
                            .await?;
                    }
                    Err(error) => self.error_response(error).await?,
                }
            }
            Some(PacketType::ReadVecF32) => {
                let packet = RequestReadVecF32MemoryPacket::deserialize(&msg);

                match self.memory.read_vec_f32(packet.address, packet.count as usize) {
                    Ok(result) => {
                        self.stream
                            .write_all(&ReceiveReadVecF32PacketResponse::serialize(result))
                            .await?;
                    }
                    Err(error) => self.error_response(error).await?,
                }
            }
            Some(PacketType::ReadU64) => {
                let packet = RequestReadU64MemoryPacket::deserialize(&msg);

                match self.memory.read_u64(packet.address, packet.size as usize) {
                    Ok(result) => {
                        self.stream
                            .write_all(&ReceiveReadU64PacketResponse::serialize(result))
                            .await?;
                    }
                    Err(error) => self.error_response(error).await?,
                }
            }
            Some(PacketType::ReadI64) => {
                let packet = RequestReadI64MemoryPacket::deserialize(&msg);

                match self.memory.read_i64(packet.address, packet.size as usize) {
                    Ok(result) => {
                        self.stream
                            .write_all(&ReceiveReadI64PacketResponse::serialize(result))
                            .await?;
                    }
                    Err(error) => self.error_response(error).await?,
                }
            }
            Some(PacketType::Write) => {
                let packet = RequestWriteVecMemoryPacket::deserialize(&msg);

                match self.memory.write(packet.address, &packet.bytes) {
                    Ok(result) => {
                        self.stream
                            .write_all(&RequestWriteVecMemoryPacketResponse::serialize(result as u64))
                            .await?;
                    }
                    Err(error) => self.error_response(error).await?,
                }
            }
            Some(PacketType::TargetPID) => {
                let packet = RequestPidRegionsPacket::deserialize(&msg);
                self.set_target_pid(packet.target_pid);

                match get_regions(packet.target_pid) {
                    Ok(regions) => {
                        self.stream
                            .write_all(&RequestPidRegionsPacketResponse::serialize(regions))
                            .await?;
                    }
                    Err(error) => self.error_response(error).await?,
                }
            }
            Some(PacketType::SendProcesses) => match get_running_processes() {
                Ok(processes) => {
                    let processes_packet = RequestProcessesPacketResponse::serialize(processes);
                    info!("processes: {:?}", processes_packet.len());
                    self.stream
                        .write_all(&processes_packet)
                        .await?;
                }
                Err(error) => self.error_response(error).await?,
            },
            _ => warn!("Unknown packet type"),
        };

        Ok(())
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
