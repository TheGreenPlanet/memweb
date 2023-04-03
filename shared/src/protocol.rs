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

pub fn parse_read_memory_packet(data: &[u8]) {
    let (rest, value) = C2SReadMemoryPacket::from_bytes((data, 1)).unwrap();
}

// pub fn write_packet() -> Vec<u8> {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_read_memory_packet() {
        let test_value: u64 = 1337;
        let mut data = vec![0x00];
        data.extend_from_slice(&test_value.to_be_bytes());

        let (_rest, mut val) = C2SReadMemoryPacket::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(
            C2SReadMemoryPacket {
                _type: PacketType::Read,
                address: 1337
            },
            val
        );
    }

    #[test]
    fn test_construct_and_parse_packet() {}
}
