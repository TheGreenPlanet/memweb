use std::io;

use deku::prelude::*;
use super::{PacketType, ErrorPacket};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestWriteVecMemoryPacket {
    _type: PacketType,
    pub address: u64,
    pub count: u32,
    #[deku(count = "count")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestWriteVecMemoryPacketResponse {
    _type: PacketType,
    pub bytes_written: u64,
}



impl RequestWriteVecMemoryPacket {
    pub fn deserialize(data: &[u8]) -> Result<Self, io::Error> {
        let packet_type = PacketType::from_u8(data[0])
            .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid packet type"))?;
        match packet_type {
            PacketType::Write => {
                let (_, value) = RequestWriteVecMemoryPacket::from_bytes((data, 0)).unwrap();
                Ok(value)
            }
            PacketType::Error => {
                let packet = ErrorPacket::deserialize(&data);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    packet.message.to_string(),
                ))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Incorrect packet type".to_string(),
            )),
        }
    }

    pub fn serialize(address: u64, bytes: Vec<u8>) -> Vec<u8> {
        let object = RequestWriteVecMemoryPacket {
            _type: PacketType::Write,
            address,
            count: bytes.len() as u32,
            bytes,
        };
        object.to_bytes().unwrap()
    }
}

impl RequestWriteVecMemoryPacketResponse {
    pub fn deserialize(data: &[u8]) -> Result<Self, io::Error> {
        let packet_type = PacketType::from_u8(data[0])
            .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid packet type"))?;
        match packet_type {
            PacketType::Write => {
                let (_, value) = RequestWriteVecMemoryPacketResponse::from_bytes((data, 0)).unwrap();
                Ok(value)
            }
            PacketType::Error => {
                let packet = ErrorPacket::deserialize(&data);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    packet.message.to_string(),
                ))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Incorrect packet type".to_string(),
            )),
        }
    }
    pub fn serialize(bytes_written: u64) -> Vec<u8> {
        let object = RequestWriteVecMemoryPacketResponse {
            _type: PacketType::Write,
            bytes_written,
        };
        object.to_bytes().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_memory_packet() {
        let data = RequestWriteVecMemoryPacket::serialize(1337, vec![123, 255]);
        let packet = RequestWriteVecMemoryPacket::deserialize(&data).unwrap();

        assert_eq!(
            RequestWriteVecMemoryPacket {
                _type: PacketType::Write,
                address: 1337,
                count: 2,
                bytes: vec![123, 255],
            },
            packet
        );
    }

    #[test]
    fn test_write_memory_packet_response() {
        const BYTES_WRITTEN: u64 = 100;
        let response_data = RequestWriteVecMemoryPacketResponse::serialize(BYTES_WRITTEN);
        let parsed_response = RequestWriteVecMemoryPacketResponse::deserialize(&response_data).unwrap();

        assert_eq!(
            RequestWriteVecMemoryPacketResponse {
                _type: PacketType::Write,
                bytes_written: 100,
            },
            parsed_response
        );
    }
}
