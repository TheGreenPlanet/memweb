#[allow(dead_code)]

use crate::protocol::{read_primitives::*, write_primitives::*};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;


async fn send_packet(stream: &mut TcpStream, packet: Vec<u8>) -> io::Result<Vec<u8>> {
    stream.write_all(&packet).await?;
    // Wait and read the response
    let mut buffer = vec![0; 1026];
    let bytes_read = stream.read(&mut buffer).await?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

pub struct TCPMemory<'a> {
    pub stream: &'a mut TcpStream,
}

union UResult {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
}

union IResult {
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
}

impl<'a> TCPMemory<'a> {
    pub fn new(stream: &'a mut TcpStream) -> TCPMemory<'a> {
        TCPMemory {
            stream,
        }
    }

    pub async fn read_vec_f32(&mut self, address: u64, count: u8) -> io::Result<Vec<f32>> {
        let response = send_packet(
            self.stream,
            RequestReadVecF32MemoryPacket::serialize(address, count),
        )
        .await?;
        // Construct UResult based on width_bytes
        Ok(ReceiveReadVecF32PacketResponse::deserialize(&response).data)
    }

    pub async fn read_vec(&mut self, address: u64, size: u32) -> io::Result<Vec<u8>> {
        let response = send_packet(
            self.stream,
            RequestReadVecMemoryPacket::serialize(address, size),
        )
        .await?;
        // Construct UResult based on width_bytes
        Ok(ReceiveReadVecPacketResponse::deserialize(&response).data)
    }

    async fn read_unsigned(&mut self, address: u64, width_bytes: u8) -> io::Result<UResult> {
        let response = send_packet(
            self.stream,
            RequestReadU64MemoryPacket::serialize(address, width_bytes),
        )
        .await?;

        let value = ReceiveReadU64PacketResponse::deserialize(&response).value;

        // Construct UResult based on width_bytes
        Ok(match width_bytes {
            1 => UResult { u8: value as u8 },
            2 => UResult { u16: value as u16 },
            4 => UResult { u32: value as u32 },
            8 => UResult { u64: value },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported byte width")),
        })
    }

    async fn read_signed(&mut self, address: u64, width_bytes: u8) -> io::Result<IResult> {
        let response = send_packet(
            self.stream,
            RequestReadI64MemoryPacket::serialize(address, width_bytes),
        )
        .await?;

        let value = ReceiveReadI64PacketResponse::deserialize(&response).value;

        // Construct IResult based on width_bytes
        Ok(match width_bytes {
            1 => IResult { i8: value as i8 },
            2 => IResult { i16: value as i16 },
            4 => IResult { i32: value as i32 },
            8 => IResult { i64: value },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported byte width")),
        })
    }

    pub async fn read_u8(&mut self, address: u64) -> io::Result<u8> {
        let r = self.read_unsigned(address, 1).await?;
        unsafe { Ok(r.u8) }
    }

    pub async fn read_u16(&mut self, address: u64) -> io::Result<u16> {
        let r = self.read_unsigned(address, 2).await?;
        unsafe { Ok(r.u16) }
    }

    pub async fn read_u32(&mut self, address: u64) -> io::Result<u32> {
        let r = self.read_unsigned(address, 4).await?;
        unsafe { Ok(r.u32) }
    }

    pub async fn read_u64(&mut self, address: u64) -> io::Result<u64> {
        let r = self.read_unsigned(address, 8).await?;
        unsafe { Ok(r.u64) }
    }

    pub async fn read_i8(&mut self, address: u64) -> io::Result<i8> {
        let r = self.read_signed(address, 1).await?;
        unsafe { Ok(r.i8) }
    }

    pub async fn read_i16(&mut self, address: u64) -> io::Result<i16> {
        let r = self.read_signed(address, 2).await?;
        unsafe { Ok(r.i16) }
    }

    pub async fn read_i32(&mut self, address: u64) -> io::Result<i32> {
        let r = self.read_signed(address, 4).await?;
        unsafe { Ok(r.i32) }
    }

    pub async fn read_i64(&mut self, address: u64) -> io::Result<i64> {
        let r = self.read_signed(address, 8).await?;
        unsafe { Ok(r.i64) }
    }

    // pub async fn read_f32(&mut self, address: u64) -> io::Result<f32> {
    //     self.read::<f32>(address).await
    // }

    // pub async fn read_f64(&mut self, address: u64) -> io::Result<f64> {
    //     self.read::<f64>(address).await
    // }

    // pub async fn read_vec3(&mut self, address: u64) -> io::Result<f64> {
    //     self.read::<f64>(address).await
    // }


    pub async fn read_ptr(&mut self, address: u64) -> io::Result<Option<u64>> {
        let r = self.read_u64(address).await?;
        if r == 0 {
            return Ok(None)
        }
        Ok(Some(r))
    }


    pub async fn write(&mut self, address: u64, data: Vec<u8>) -> io::Result<u64> {
        let response = send_packet(
            self.stream,
            RequestWriteVecMemoryPacket::serialize(address, data),
        )
        .await?;
        Ok(RequestWriteVecMemoryPacketResponse::deserialize(&response).bytes_written)
    }
}