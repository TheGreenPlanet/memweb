#![allow(non_snake_case)]
use std::vec;

use deku::prelude::*;

type Pid = i32;

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
    pub size: u32,
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SGetProcessesPacket {
    _type: PacketType,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ProcessEntry {
    pub name: EncodedString,
    pub pid: Pid,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct S2CSendProcessesPacket {
    _type: PacketType,
    #[deku(update = "self.processes.len() as u32")]
    pub count: u32,
    #[deku(count = "count")]
    pub processes: Vec<ProcessEntry>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct S2CReadMemoryPacketResponse {
    _type: PacketType,
    pub count: u32,
    #[deku(count = "count")]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct S2CWriteMemoryPacketResponse {
    _type: PacketType,
    pub bytes_written: u64,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct EncodedString {
    pub length: u32,
    #[deku(count = "length")]
    pub string: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Region {
    pub start: u64,
    pub end: u64,
    pub size: u64,
    pub permissions: u8,
    pub offset: u64,
    pub device: EncodedString,
    pub inode: u64,
    pub pathname: EncodedString,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct S2CTargetPidRegionsPacket {
    _type: PacketType,
    #[deku(update = "self.regions.len() as u32")]
    pub count: u32,
    #[deku(count = "count")]
    pub regions: Vec<Region>,
}

impl EncodedString {
    //todo!("Avoid copying the string");
    pub fn new(string: String) -> Self {
        Self {
            length: string.len() as u32,
            string: string.as_bytes().to_vec(),
        }
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.string.clone()).unwrap()
    }
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

    pub fn out_bytes(target_pid: Pid) -> Vec<u8> {
        let object = C2STargetPidPacket {
            _type: PacketType::TargetPID,
            target_pid,
        };
        object.to_bytes().unwrap()
    }
}

impl C2SReadMemoryPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2SReadMemoryPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(address: u64, size: u32) -> Vec<u8> {
        let object = C2SReadMemoryPacket {
            _type: PacketType::Read,
            address,
            size,
        };
        object.to_bytes().unwrap()
    }
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

impl C2SGetProcessesPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = C2SGetProcessesPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes() -> Vec<u8> {
        let object = C2SGetProcessesPacket {
            _type: PacketType::SendProcesses,
        };
        object.to_bytes().unwrap()
    }
}

impl S2CReadMemoryPacketResponse {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = S2CReadMemoryPacketResponse::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(data: Vec<u8>) -> Vec<u8> {
        let object = S2CReadMemoryPacketResponse {
            _type: PacketType::Read,
            count: data.len() as u32,
            data,
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

impl S2CTargetPidRegionsPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = S2CTargetPidRegionsPacket::from_bytes((data, 0)).unwrap();
        value
    }
    pub fn out_bytes(regions: Vec<Region>) -> Vec<u8> {
        let object = S2CTargetPidRegionsPacket {
            _type: PacketType::TargetPID,
            count: regions.len() as u32,
            regions: regions,
        };
        object.to_bytes().unwrap()
    }
}

impl S2CSendProcessesPacket {
    pub fn parse(data: &[u8]) -> Self {
        let (_, value) = S2CSendProcessesPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn out_bytes(processes: Vec<ProcessEntry>) -> Vec<u8> {
        let object = S2CSendProcessesPacket {
            _type: PacketType::SendProcesses,
            count: processes.len() as u32,
            processes: processes,
        };
        object.to_bytes().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_memory_packet() {
        let data = C2SReadMemoryPacket::out_bytes(1337, 100);
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
    fn test_target_pid_packet() {
        let data = C2STargetPidPacket::out_bytes(1234567890);
        let packet = C2STargetPidPacket::parse(&data);

        assert_eq!(
            C2STargetPidPacket {
                _type: PacketType::TargetPID,
                target_pid: 1234567890,
            },
            packet
        );
    }

    #[test]
    fn test_read_memory_packet_response() {
        let test_payload = vec![255, 100, 50, 25, 10];

        let response_data = S2CReadMemoryPacketResponse::out_bytes(test_payload);
        let parsed_response = S2CReadMemoryPacketResponse::parse(&response_data);

        assert_eq!(
            S2CReadMemoryPacketResponse {
                _type: PacketType::Read,
                count: 5,
                data: vec![255, 100, 50, 25, 10],
            },
            parsed_response
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

    #[test]
    fn test_target_pid_regions() {
            
        let test_regions = vec![
            Region {
                start: 0x0000555555554000,
                end: 0x0000555555555000,
                size: 4096,
                permissions: 5,
                offset: 0,
                device: EncodedString::new("major:minor".to_string()),
                inode: 0,
                pathname: EncodedString::new("/home/username/Projects/memflow-web-service/target/debug/memflow-web-service".to_string()),
            },
            Region {
                start: 0x00007ffff7dc0000,
                end: 0x00007ffff7dc1000,
                size: 4096,
                permissions: 4,
                offset: 0,
                device: EncodedString::new("minor:major".to_string()),
                inode: 0,
                pathname: EncodedString::new("/home/username/Projects/memflow-web-service/target/debug/memflow-web-service".to_string()),
            },
        ];
        let data = S2CTargetPidRegionsPacket::out_bytes(test_regions);
        let parsed_response = S2CTargetPidRegionsPacket::parse(&data);

        assert_eq!(
            S2CTargetPidRegionsPacket {
                _type: PacketType::TargetPID,
                count: 2,
                regions: vec![
                    Region {
                        start: 0x0000555555554000,
                        end: 0x0000555555555000,
                        size: 4096,
                        permissions: 5,
                        offset: 0,
                        device: EncodedString::new("major:minor".to_string()),
                        inode: 0,
                        pathname: EncodedString::new("/home/username/Projects/memflow-web-service/target/debug/memflow-web-service".to_string()),
                    },
                    Region {
                        start: 0x00007ffff7dc0000,
                        end: 0x00007ffff7dc1000,
                        size: 4096,
                        permissions: 4,
                        offset: 0,
                        device: EncodedString::new("minor:major".to_string()),
                        inode: 0,
                        pathname: EncodedString::new("/home/username/Projects/memflow-web-service/target/debug/memflow-web-service".to_string()),
                    },
                ],
            },
            parsed_response
        );
    }

    #[test]
    fn test_send_processes_packet() {

        let test_processes = vec![
            ProcessEntry {
                name: EncodedString::new("memflow-web-service".to_string()),
                pid: 1234567890,
            },
            ProcessEntry {
                name: EncodedString::new("memflow-web-service-2".to_string()),
                pid: 0987654321,
            },
        ];

        let data = S2CSendProcessesPacket::out_bytes(test_processes);
        let packet = S2CSendProcessesPacket::parse(&data);

        assert_eq!(
            S2CSendProcessesPacket {
                _type: PacketType::SendProcesses,
                count: 2,
                processes: vec![
                    ProcessEntry {
                        name: EncodedString::new("memflow-web-service".to_string()),
                        pid: 1234567890,
                    },
                    ProcessEntry {
                        name: EncodedString::new("memflow-web-service-2".to_string()),
                        pid: 0987654321,
                    },
                ],
            },
            packet
        );
    }
}
