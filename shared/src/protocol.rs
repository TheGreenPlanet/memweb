use deku::prelude::*;

type Pid = u32;

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
    pub address: u64,
    pub size: usize,
}

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
pub struct C2STargetPidPacket {
    _type: PacketType,
    pub target_pid: Pid,
}

// #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
// #[deku(endian = "big")]
// pub struct EncodedString {
//     length: u32,
//     #[deku(count = "length")]
//     string: Vec<u8>,
// }

// #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
// #[deku(endian = "big")]
// pub struct S2CSendProcessesPacket {
//     _type: PacketType,
//     #[deku(update = "self.processes.len() as u32")]
//     count: u32,
//     #[deku(count = "count")]
//     processes: Vec<(EncodedString, u32)>
// }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SReadMemoryPacketResponse {
    _type: PacketType,
    pub count: usize,
    #[deku(count = "count")]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SWriteMemoryPacketResponse {
    _type: PacketType,
    pub bytes_written: usize,
}

impl PacketType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Read),
            1 => Some(Self::Write),
            2 => Some(Self::TargetPID),
            3 => Some(Self::SendProcesses),
            _ => None,
        }
    }
}

impl C2STargetPidPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2STargetPidPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(_type: PacketType, target_pid: Pid) -> Vec<u8> {
        let object = C2STargetPidPacket { _type, target_pid };
        object.to_bytes().unwrap()
    }
}

impl C2SReadMemoryPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2SReadMemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(_type: PacketType, address: u64, size: usize) -> Vec<u8> {
        let object = C2SReadMemoryPacket { _type, address, size };
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

impl C2SReadMemoryPacketResponse {
    pub fn out_bytes(data: Vec<u8>) -> Vec<u8> {
        let object = C2SReadMemoryPacketResponse { _type: PacketType::Read, count: data.len(), data  };
        object.to_bytes().unwrap()
    }
}

impl C2SWriteMemoryPacketResponse {
    pub fn out_bytes(bytes_written: usize) -> Vec<u8> {
        let object = C2SWriteMemoryPacketResponse { _type: PacketType::Write, bytes_written };
        object.to_bytes().unwrap()
    }
}


// impl S2CSendProcessesPacket {
//     pub fn parse(data: &[u8]) -> Self {
//         let (_, value) = S2CSendProcessesPacket::from_bytes((data, 0)).unwrap();
//         value
//     }

//     pub fn out_bytes(_type: PacketType, address: u64, bytes: Vec<u8>) -> Vec<u8> {
//         let object = S2CSendProcessesPacket { _type, address, count: bytes.len() as u32, bytes };
//         object.to_bytes().unwrap()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_memory_packet() {
        let data = C2SReadMemoryPacket::out_bytes(PacketType::Read, 1337, 100);
        let packet = C2SReadMemoryPacket::parse(&data);

        assert_eq!(
            C2SReadMemoryPacket {
                _type: PacketType::Read,
                address: 1337,
                size: 100,
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


    // #[test]
    // fn test_send_processes_packet() {
    //     let data = S2CSendProcessesPacket::out_bytes(PacketType::SendProcesses, 1234567890);
    //     let packet = S2CSendProcessesPacket::parse(&data);

    //     assert_eq!(
    //         S2CSendProcessesPacket {
    //             _type: PacketType::SendProcesses,
    //             target_pid: 1234567890,
    //         },
    //         packet
    //     );
    // }
}
