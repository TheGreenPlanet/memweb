use deku::prelude::*;

#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
#[deku(enum = "u8")]
pub enum PacketType {
    Read = 0,
    Write = 1,
    TargetPID = 2,
    SendProcesses = 3,
    Error = 4,
}

#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SReadMemoryPacket {
    _type: PacketType,
    address: u64,
}

#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2SWriteMemoryPacket {
    _type: PacketType,
    address: u64,
    count: u32,
    #[deku(count = "count")]
    data: Vec<u8>,
}

#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct C2STargetPidPacket {
    _type: PacketType,
    target_pid: u32,
}

#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct S2CSendProcessesPacket {
    _type: PacketType,
    #[deku(update = "self.processes.len() as u32")]
    count: u32,
    #[deku(count = "count")]
    processes: Vec<(String, u32)>
}


pub fn parse_read_memory_packet(data: &[u8]) {
    let (rest, value) = C2SReadMemoryPacket::from_bytes((data, 1)).unwrap();
}

pub fn write_packet() -> Vec<u8> {
    
}