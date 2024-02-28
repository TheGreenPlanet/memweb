#![allow(non_snake_case)]
use deku::prelude::*;
use crate::compression;

pub mod read_primitives;
pub mod write_primitives;


type Pid = i32;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")] // context passed from `DekuTest` top-level endian
pub enum PacketType {
    ReadVec = 0,
    ReadU64,
    ReadI64,
    Write,
    TargetPID,
    SendProcesses,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestPidRegionsPacket {
    _type: PacketType,
    pub target_pid: Pid,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestProcessesPacket {
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
pub struct RequestProcessesPacketResponse {
    _type: PacketType,
    #[deku(update = "self.processes.len() as u32")]
    pub count: u32,
    #[deku(count = "count")]
    pub processes: Vec<ProcessEntry>,
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
pub struct RequestPidRegionsPacketResponse {
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
            0 => Some(Self::ReadVec),
            1 => Some(Self::ReadU64),
            2 => Some(Self::ReadI64),
            3 => Some(Self::Write),
            4 => Some(Self::TargetPID),
            5 => Some(Self::SendProcesses),
            _ => None,
        }
    }
}

impl RequestPidRegionsPacket {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = RequestPidRegionsPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize(target_pid: Pid) -> Vec<u8> {
        let object = RequestPidRegionsPacket {
            _type: PacketType::TargetPID,
            target_pid,
        };
        object.to_bytes().unwrap()
    }
}


impl RequestProcessesPacket {
    pub fn deserialize(data: &[u8]) -> Self {
        let (_, value) = RequestProcessesPacket::from_bytes((data, 0)).unwrap();
        value
    }

    pub fn serialize() -> Vec<u8> {
        let object = RequestProcessesPacket {
            _type: PacketType::SendProcesses,
        };
        object.to_bytes().unwrap()
    }
}


impl RequestPidRegionsPacketResponse {
    pub fn deserialize(data: &[u8]) -> Self {
        let decompressed_data = compression::decompress(&data);
        let (_, value) = RequestPidRegionsPacketResponse::from_bytes((decompressed_data.as_ref(), 0)).unwrap();
        value
    }
    pub fn serialize(regions: Vec<Region>) -> Vec<u8> {
        let object = RequestPidRegionsPacketResponse {
            _type: PacketType::TargetPID,
            count: regions.len() as u32,
            regions: regions,
        };

        compression::compress(object.to_bytes().unwrap().as_ref())
    }
}

impl RequestProcessesPacketResponse {
    pub fn deserialize(data: &[u8]) -> Self {
        let decompressed_data = compression::decompress(&data);
        let (_, value) = RequestProcessesPacketResponse::from_bytes((decompressed_data.as_ref(), 0)).unwrap();
        value
    }

    pub fn serialize(processes: Vec<ProcessEntry>) -> Vec<u8> {
        let object = RequestProcessesPacketResponse {
            _type: PacketType::SendProcesses,
            count: processes.len() as u32,
            processes: processes,
        };
        compression::compress(object.to_bytes().unwrap().as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_pid_packet() {
        let data = RequestPidRegionsPacket::serialize(1234567890);
        let packet = RequestPidRegionsPacket::deserialize(&data);

        assert_eq!(
            RequestPidRegionsPacket {
                _type: PacketType::TargetPID,
                target_pid: 1234567890,
            },
            packet
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
        let data = RequestPidRegionsPacketResponse::serialize(test_regions);
        let parsed_response = RequestPidRegionsPacketResponse::deserialize(&data);

        assert_eq!(
            RequestPidRegionsPacketResponse {
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

        let data = RequestProcessesPacketResponse::serialize(test_processes);
        let packet = RequestProcessesPacketResponse::deserialize(&data);

        assert_eq!(
            RequestProcessesPacketResponse {
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
