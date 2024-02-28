use deku::prelude::*;
use super::PacketType;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SWriteMemoryPacket {
    _type: PacketType,
    pub address: u64,
    pub count: u32,
    #[deku(count = "count")]
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct S2CWriteMemoryPacketResponse {
    _type: PacketType,
    pub bytes_written: u64,
}



impl C2SWriteMemoryPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2SWriteMemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(address: u64, bytes: Vec<u8>) -> Vec<u8> {
        let object = C2SWriteMemoryPacket {
            _type: PacketType::Write,
            address,
            count: bytes.len() as u32,
            bytes,
        };
        object.to_bytes().unwrap()
    }
}

impl S2CWriteMemoryPacketResponse {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = S2CWriteMemoryPacketResponse::from_bytes((data, 0)).unwrap();
        value
    }
    pub fn out_bytes(bytes_written: u64) -> Vec<u8> {
        let object = S2CWriteMemoryPacketResponse {
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
        let data = C2SWriteMemoryPacket::out_bytes(1337, vec![123, 255]);
        let packet = C2SWriteMemoryPacket::parse(&data);

        assert_eq!(
            C2SWriteMemoryPacket {
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
        let response_data = S2CWriteMemoryPacketResponse::out_bytes(BYTES_WRITTEN);
        let parsed_response = S2CWriteMemoryPacketResponse::parse(&response_data);

        assert_eq!(
            S2CWriteMemoryPacketResponse {
                _type: PacketType::Write,
                bytes_written: 100,
            },
            parsed_response
        );
    }
}
