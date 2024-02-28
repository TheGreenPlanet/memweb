use shared::protocol::{read_primitives::*, write_primitives::*, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn parse_payload_to_string(msg: &[u8]) -> String {
    if msg.is_empty() {
        return "Empty message".to_string();
    }
    match PacketType::from_u8(msg[0]) {
        Some(PacketType::ReadVec) => {
            let packet = ReceiveReadVecPacketResponse::deserialize(&msg);
            format!("Read: count: {}, data: {:?}", packet.count, packet.data)
        }
        Some(PacketType::ReadU64) => {
            let packet = ReceiveReadU64PacketResponse::deserialize(&msg);
            format!("ReadU64: {}", packet.value)
        }
        Some(PacketType::ReadI64) => {
            let packet = ReceiveReadI64PacketResponse::deserialize(&msg);
            format!("ReadI64: {}", packet.value)
        }
        Some(PacketType::Write) => {
            let packet = RequestWriteVecMemoryPacketResponse::deserialize(&msg);
            format!("Write: bytes written: {}", packet.bytes_written)
        }
        Some(PacketType::TargetPID) => {
            let packet = RequestPidRegionsPacketResponse::deserialize(&msg);
            let regions_string = packet.regions.iter().fold(String::new(), |acc, region| {
                acc + &format!("Start: {}, End: {}, Size: {}, Permissions: {}, Offset: {}, Device: {}, Inode: {}, Pathname: {}\n", region.start, region.end, region.size, region.permissions, region.offset, region.device.to_string(), region.inode, region.pathname.to_string())
            });
            format!(
                "TargetPID: count: {}, regions: {}\n",
                packet.count, regions_string
            )
        }
        Some(PacketType::SendProcesses) => {
            let packet = RequestProcessesPacketResponse::deserialize(&msg);
            let processes = packet.processes.iter().fold(String::new(), |acc, process| {
                acc + &format!("Pid: {}, Name: {}\n", process.pid, process.name.to_string())
            });
            format!(
                "SendProcesses: count: {} processes: {}\n",
                packet.count, processes
            )
        }
        None => {
            format!("None")
        }
    }
}

#[wasm_bindgen]
pub fn target_pid_packet_data(pid: i32) -> Vec<u8> {
    RequestPidRegionsPacket::serialize(pid)
}

#[wasm_bindgen]
pub fn read_memory_packet_data(address: u64, size: u32) -> Vec<u8> {
    RequestReadVecMemoryPacket::serialize(address, size)
}

#[wasm_bindgen]
pub fn read_memory_u64_packet_data(address: u64, size: u8) -> Vec<u8> {
    RequestReadU64MemoryPacket::serialize(address, size)
}

#[wasm_bindgen]
pub fn read_memory_i64_packet_data(address: u64, size: u8) -> Vec<u8> {
    RequestReadI64MemoryPacket::serialize(address, size)
}

#[wasm_bindgen]
pub fn write_memory_packet_data(address: u64, bytes: &[u8]) -> Vec<u8> {
    RequestWriteVecMemoryPacket::serialize(address, bytes.to_vec())
}

#[wasm_bindgen]
pub fn get_processes_packet_data() -> Vec<u8> {
    RequestProcessesPacket::serialize()
}
