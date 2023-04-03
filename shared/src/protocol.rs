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
// #[deku(endian = "big")]
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
    data: Vec<u8>,
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

impl C2SReadMemoryPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2SReadMemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }
    
    pub fn out_bytes(_type: PacketType, address: u64) -> Vec<u8> {
        let mut data = vec![_type as u8];
        data.extend_from_slice(&address.to_be_bytes());
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_read_memory_packet() {
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
    fn test_construct_and_parse_packet() {

    }
}
