use deku::prelude::*;
use super::PacketType;

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
pub struct ReceiveWriteVecMemoryPacketResponse {
    _type: PacketType,
    pub bytes_written: u64,
}



impl RequestWriteVecMemoryPacket {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = RequestWriteVecMemoryPacket::from_bytes((data, 0)).unwrap();
        value
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

impl ReceiveWriteVecMemoryPacketResponse {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = ReceiveWriteVecMemoryPacketResponse::from_bytes((data, 0)).unwrap();
        value
    }
    pub fn out_bytes(bytes_written: u64) -> Vec<u8> {
        let object = ReceiveWriteVecMemoryPacketResponse {
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
        let packet = RequestWriteVecMemoryPacket::deserialize(&data);

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
        let response_data = ReceiveWriteVecMemoryPacketResponse::out_bytes(BYTES_WRITTEN);
        let parsed_response = ReceiveWriteVecMemoryPacketResponse::parse(&response_data);

        assert_eq!(
            ReceiveWriteVecMemoryPacketResponse {
                _type: PacketType::Write,
                bytes_written: 100,
            },
            parsed_response
        );
    }
}
