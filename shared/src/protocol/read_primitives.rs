use deku::prelude::*;
use super::PacketType;


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestReadVecMemoryPacket {
    _type: PacketType,
    pub address: u64,
    pub size: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestReadU64MemoryPacket {
    _type: PacketType,
    pub address: u64,
    pub size: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestReadI64MemoryPacket {
    _type: PacketType,
    pub address: u64,
    pub size: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ReceiveReadVecPacketResponse {
    _type: PacketType,
    pub count: u32,
    #[deku(count = "count")]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ReceiveReadU64PacketResponse {
    _type: PacketType,
    pub value: u64,
}


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ReceiveReadI64PacketResponse {
    _type: PacketType,
    pub value: i64,
}

impl RequestReadVecMemoryPacket {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = RequestReadVecMemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize(address: u64, size: u32) -> Vec<u8> {
        let object = RequestReadVecMemoryPacket {
            _type: PacketType::ReadVec,
            address,
            size,
        };
        object.to_bytes().unwrap()
    }
}

impl RequestReadU64MemoryPacket {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = RequestReadU64MemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize(address: u64, size: u8) -> Vec<u8> {
        let object = RequestReadU64MemoryPacket {
            _type: PacketType::ReadU64,
            address,
            size,
        };
        object.to_bytes().unwrap()
    }
}

impl RequestReadI64MemoryPacket {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = RequestReadI64MemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize(address: u64, size: u8) -> Vec<u8> {
        let object = RequestReadI64MemoryPacket {
            _type: PacketType::ReadI64,
            address,
            size,
        };
        object.to_bytes().unwrap()
    }
}

impl ReceiveReadVecPacketResponse {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = ReceiveReadVecPacketResponse::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize(data: Vec<u8>) -> Vec<u8> {
        let object = ReceiveReadVecPacketResponse {
            _type: PacketType::ReadVec,
            count: data.len() as u32,
            data,
        };
        object.to_bytes().unwrap()
    }
}

impl ReceiveReadU64PacketResponse {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = ReceiveReadU64PacketResponse::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize(value: u64) -> Vec<u8> {
        let object = ReceiveReadU64PacketResponse {
            _type: PacketType::ReadU64,
            value,
        };
        object.to_bytes().unwrap()
    }
}


impl ReceiveReadI64PacketResponse {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = ReceiveReadI64PacketResponse::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize(value: i64) -> Vec<u8> {
        let object = ReceiveReadI64PacketResponse {
            _type: PacketType::ReadI64,
            value,
        };
        object.to_bytes().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_memory_packet() {
        let data = RequestReadVecMemoryPacket::serialize(1337, 100);
        let packet = RequestReadVecMemoryPacket::deserialize(&data);

        assert_eq!(
            RequestReadVecMemoryPacket {
                _type: PacketType::ReadVec,
                address: 1337,
                size: 100,
            },
            packet
        );
    }

    #[test]
    fn test_read_memory_packet_response() {
        let test_payload = vec![255, 100, 50, 25, 10];

        let response_data = ReceiveReadVecPacketResponse::serialize(test_payload);
        let parsed_response = ReceiveReadVecPacketResponse::deserialize(&response_data);

        assert_eq!(
            ReceiveReadVecPacketResponse {
                _type: PacketType::ReadVec,
                count: 5,
                data: vec![255, 100, 50, 25, 10],
            },
            parsed_response
        );
    }

    #[test]
    fn test_read_u64_memory_packet() {
        let data = RequestReadU64MemoryPacket::serialize(1337, 8);
        let packet = RequestReadU64MemoryPacket::deserialize(&data);

        assert_eq!(
            RequestReadU64MemoryPacket {
                _type: PacketType::ReadU64,
                address: 1337,
                size: 8,
            },
            packet
        );
    }

    #[test]
    fn test_read_u64_memory_packet_response() {
        
        let byte: u8 = 0xFF;
        let response_data = ReceiveReadU64PacketResponse::serialize(byte as u64);
        let res = ReceiveReadU64PacketResponse::deserialize(&response_data);

        assert_eq!(res.value as u8, byte);

        assert_eq!(
            ReceiveReadU64PacketResponse {
                _type: PacketType::ReadU64,
                value: 0xFF,
            },
            res
        );
    }

    #[test]
    fn test_read_i64_memory_packet() {
        let data = RequestReadI64MemoryPacket::serialize(1337, 8);
        let packet = RequestReadI64MemoryPacket::deserialize(&data);

        assert_eq!(
            RequestReadI64MemoryPacket {
                _type: PacketType::ReadI64,
                address: 1337,
                size: 8,
            },
            packet
        );
    }

    #[test]
    fn test_read_i64_memory_packet_response_smallest() {
        
        let byte: i8 = -128;
        let response_data = ReceiveReadI64PacketResponse::serialize(byte as i64);
        let res = ReceiveReadI64PacketResponse::deserialize(&response_data);

        assert_eq!(res.value as i8, byte);

        assert_eq!(
            ReceiveReadI64PacketResponse {
                _type: PacketType::ReadI64,
                value: -128,
            },
            res
        );
    }

    #[test]
    fn test_read_i64_memory_packet_response_largest() {
        
        let byte: i8 = 127;
        let response_data = ReceiveReadI64PacketResponse::serialize(byte as i64);
        let res = ReceiveReadI64PacketResponse::deserialize(&response_data);

        assert_eq!(res.value as i8, byte);

        assert_eq!(
            ReceiveReadI64PacketResponse {
                _type: PacketType::ReadI64,
                value: 127,
            },
            res
        );
    }
}
