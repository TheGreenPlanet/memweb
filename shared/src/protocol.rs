use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")] // context passed from `DekuTest` top-level endian
pub enum PacketType {
    Read = 0,
    Write = 1,
    TargetPID = 2,
    SendProcesses = 3,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SReadMemoryPacket {
    _type: PacketType,
    address: u64,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SWriteMemoryPacket {
    _type: PacketType,
    address: u64,
    count: u32,
    #[deku(count = "count")]
    bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2STargetPidPacket {
    _type: PacketType,
    target_pid: u32,
}

// #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
// pub struct S2CSendProcessesPacket {
//     _type: PacketType,
//     #[deku(update = "self.processes.len() as u32")]
//     count: u32,
//     #[deku(count = "count")]
//     processes: Vec<(String, u32)>
// }

impl C2STargetPidPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2STargetPidPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(_type: PacketType, target_pid: u32) -> Vec<u8> {
        let object = C2STargetPidPacket { _type, target_pid };
        object.to_bytes().unwrap()
    }
}

impl C2SReadMemoryPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2SReadMemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(_type: PacketType, address: u64) -> Vec<u8> {
        let object = C2SReadMemoryPacket { _type, address };
        object.to_bytes().unwrap()
    }
}

impl C2SWriteMemoryPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2SWriteMemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(_type: PacketType, address: u64, bytes: Vec<u8>) -> Vec<u8> {
        let object = C2SWriteMemoryPacket { _type, address, count: bytes.len() as u32, bytes };
        object.to_bytes().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_memory_packet() {
        let data = C2SReadMemoryPacket::out_bytes(PacketType::Read, 1337);
        let packet = C2SReadMemoryPacket::parse(&data);

        assert_eq!(
            C2SReadMemoryPacket {
                _type: PacketType::Read,
                address: 1337
            },
            packet
        );
    }

    #[test]
    fn test_write_memory_packet() {
        let data = C2SWriteMemoryPacket::out_bytes(PacketType::Write, 1337, vec![123, 255]);
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
    fn test_target_pid_packet() {
        let data = C2STargetPidPacket::out_bytes(PacketType::TargetPID, 1234567890);
        let packet = C2STargetPidPacket::parse(&data);

        assert_eq!(
            C2STargetPidPacket {
                _type: PacketType::TargetPID,
                target_pid: 1234567890,
            },
            packet
        );
    }
}
