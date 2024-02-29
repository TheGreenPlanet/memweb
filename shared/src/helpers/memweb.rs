#[allow(dead_code)]
use crate::protocol::*;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct MemWeb<'a> {
    pub stream: &'a mut TcpStream,
}

impl<'a> MemWeb<'a> {
    pub fn new(stream: &'a mut TcpStream) -> MemWeb<'a> {
        MemWeb { stream }
    }

    pub async fn get_processes(&mut self) -> io::Result<Vec<ProcessEntry>> {
        let data = self.send_packet_big_read(RequestProcessesPacket::serialize()).await?;
        if data.len() == 0 {
            panic!("Process list is empty!")
        }

        let processes = RequestProcessesPacketResponse::deserialize(&data).unwrap();
        Ok(processes.processes)
    }

    pub async fn get_regions(&mut self, pid: i32) -> io::Result<Vec<Region>> {
        let data = self.send_packet_big_read(RequestPidRegionsPacket::serialize(pid)).await?;
        if data.len() == 0 {
            panic!("Region list is empty!")
        }

        let regions = RequestPidRegionsPacketResponse::deserialize(&data).unwrap();
        Ok(regions.regions)
    }

    async fn send_packet_big_read(&mut self, packet: Vec<u8>) -> io::Result<Vec<u8>> {
        self.stream.write_all(&packet).await?;
        // Wait and read the response
        let mut buffer = vec![0; 100000];
        let bytes_read = self.stream.read(&mut buffer).await?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }
}
